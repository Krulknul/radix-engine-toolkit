// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at

//   http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#![allow(clippy::missing_safety_doc)]
#![allow(non_snake_case)]

pub mod native {
    use radix_engine_toolkit::functions::*;
    use radix_engine_toolkit::utils::debug_string;
    use radix_engine_toolkit::{buffer::Pointer, error::InvocationInterpretationError};
    use serde::{Deserialize, Serialize};

    pub unsafe fn deserialize_from_memory<'a, T: Deserialize<'a>>(
        string_pointer: Pointer,
    ) -> Result<T, InvocationInterpretationError> {
        std::ffi::CStr::from_ptr(string_pointer as *const std::ffi::c_char)
            .to_str()
            .map_err(InvocationInterpretationError::from)
            .and_then(|string| {
                serde_json::from_str(string).map_err(|error| {
                    InvocationInterpretationError::DeserializationError {
                        message: debug_string(error),
                    }
                })
            })
    }

    pub unsafe fn write_serializable_to_memory<T: Serialize>(
        object: &T,
    ) -> Result<Pointer, InvocationInterpretationError> {
        serde_json::to_string(object)
            .map_err(|error| InvocationInterpretationError::SerializationError {
                message: debug_string(error),
            })
            .map(|string| {
                let object_bytes = string.as_bytes();
                let byte_count = object_bytes.len() + 1;

                let pointer = radix_engine_toolkit::buffer::toolkit_alloc(byte_count);
                pointer.copy_from(
                    [object_bytes, &[0]].concat().as_ptr() as Pointer,
                    byte_count,
                );

                pointer
            })
    }

    macro_rules! export_handler {
        ($handler: ty as $handler_ident: ident) => {
            #[no_mangle]
            pub unsafe extern "C" fn $handler_ident(
                string_pointer: radix_engine_toolkit::buffer::Pointer,
            ) -> radix_engine_toolkit::buffer::Pointer {
                let result_pointers = deserialize_from_memory(string_pointer)
                    .map_err(|error| {
                        radix_engine_toolkit::error::RETError::InvocationInterpretationError(error)
                    })
                    .and_then(|request| {
                        <$handler>::fulfill(request).map_err(|error| {
                            radix_engine_toolkit::error::RETError::InvocationHandlingError(
                                error.into(),
                            )
                        })
                    })
                    .and_then(|response| {
                        write_serializable_to_memory(&response).map_err(|error| {
                            radix_engine_toolkit::error::RETError::InvocationInterpretationError(
                                error,
                            )
                        })
                    })
                    .map_err(|error| {
                        write_serializable_to_memory(&error)
                            .expect("Failed to serialize error which is a trusted object")
                    });
                match result_pointers {
                    Ok(pointer) => pointer,
                    Err(pointer) => pointer,
                }
            }
        };
    }

    export_handler!(information::Handler as information);

    export_handler!(convert_manifest::Handler as convert_manifest);
    export_handler!(extract_addresses_from_manifest::Handler as extract_addresses_from_manifest);
    #[cfg(feature = "radix-engine")]
    export_handler!(analyze_transaction_execution::Handler as analyze_transaction_execution);

    export_handler!(compile_transaction_intent::Handler as compile_transaction_intent);
    export_handler!(
        compile_signed_transaction_intent::Handler as compile_signed_transaction_intent
    );
    export_handler!(compile_notarized_transaction::Handler as compile_notarized_transaction);

    export_handler!(decompile_transaction_intent::Handler as decompile_transaction_intent);
    export_handler!(
        decompile_signed_transaction_intent::Handler as decompile_signed_transaction_intent
    );
    export_handler!(decompile_notarized_transaction::Handler as decompile_notarized_transaction);
    export_handler!(decompile_unknown_intent::Handler as decompile_unknown_transaction_intent);

    export_handler!(
        derive_babylon_address_from_olympia_address::Handler
            as derive_babylon_address_from_olympia_address
    );
    export_handler!(
        derive_olympia_address_from_public_key::Handler as derive_olympia_address_from_public_key
    );
    export_handler!(derive_virtual_account_address::Handler as derive_virtual_account_address);
    export_handler!(derive_virtual_identity_address::Handler as derive_virtual_identity_address);

    export_handler!(encode_address::Handler as encode_address);
    export_handler!(decode_address::Handler as decode_address);

    export_handler!(sbor_encode::Handler as sbor_encode);
    export_handler!(sbor_decode::Handler as sbor_decode);

    export_handler!(known_entity_addresses::Handler as known_entity_addresses);
    export_handler!(statically_validate_transaction::Handler as statically_validate_transaction);

    export_handler!(hash::Handler as hash);
}

#[cfg(feature = "jni")]
pub mod jni {
    use radix_engine_toolkit::{
        error::InvocationInterpretationError, functions::*, utils::debug_string,
    };
    use serde::Serialize;

    fn serialize_to_jstring<T: Serialize>(
        env: jni::JNIEnv,
        object: &T,
    ) -> Result<jni::sys::jstring, InvocationInterpretationError> {
        serde_json::to_string(object)
            .map_err(|error| InvocationInterpretationError::SerializationError {
                message: debug_string(error),
            })
            .and_then(|string| {
                env.new_string(&string)
                    .map_err(|_| InvocationInterpretationError::JniStringAllocationFailed)
            })
            .map(|object| object.into_raw())
    }

    macro_rules! export_handler {
        ($handler: ty as $handler_ident: ident) => {
            #[no_mangle]
            pub unsafe extern "system" fn $handler_ident(
                env: jni::JNIEnv,
                _: jni::objects::JClass,
                input: jni::objects::JString,
            ) -> jni::sys::jstring {
                let result_strings = env
                    .get_string(input)
                    .map_err(
                        |_| radix_engine_toolkit::error::InvocationInterpretationError::JniStringReadFailed,
                    )
                    .and_then(|string_object| {
                        serde_json::from_str(&String::from(string_object)).map_err(|error| {
                            InvocationInterpretationError::DeserializationError {
                                message: debug_string(error),
                            }
                        })
                    })
                    .map_err(|error| {
                        radix_engine_toolkit::error::RETError::InvocationInterpretationError(error)
                    })
                    .and_then(|request| {
                        <$handler>::fulfill(request).map_err(|error| {
                            radix_engine_toolkit::error::RETError::InvocationHandlingError(
                                error.into(),
                            )
                        })
                    })
                    .and_then(|response| serialize_to_jstring(env, &response).map_err(|error| {
                        radix_engine_toolkit::error::RETError::InvocationInterpretationError(error)
                    }))
                    .map_err(|error| {
                        serialize_to_jstring(env, &error)
                            .expect("Failed to convert a trusted payload to jstring")
                    });

                match result_strings {
                    Ok(string) => string,
                    Err(string) => string,
                }
            }
        };
    }

    export_handler!(
        information::Handler as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_information
    );

    export_handler!(
        convert_manifest::Handler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_convertManifest
    );
    export_handler!(
        extract_addresses_from_manifest::Handler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_extractAddressesFromManifest
    );
    #[cfg(feature = "radix-engine")]
    export_handler!(
        analyze_transaction_execution::Handler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_analyzeTransactionExecution
    );

    export_handler!(
        compile_transaction_intent::Handler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_compileTransactionIntent
    );
    export_handler!(
        compile_signed_transaction_intent::Handler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_compileSignedTransactionIntent
    );
    export_handler!(
        compile_notarized_transaction::Handler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_compileNotarizedTransaction
    );

    export_handler!(
        decompile_transaction_intent::Handler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_decompileTransactionIntent
    );
    export_handler!(
        decompile_signed_transaction_intent::Handler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_decompileSignedTransactionIntent
    );
    export_handler!(
        decompile_notarized_transaction::Handler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_decompileNotarizedTransaction
    );
    export_handler!(
        decompile_unknown_intent::Handler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_decompileUnknownTransactionIntent
    );

    export_handler!(
        derive_babylon_address_from_olympia_address::Handler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_deriveBabylonAddressFromOlympiaAddress
    );
    export_handler!(
        derive_olympia_address_from_public_key::Handler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_deriveOlympiaAddressFromPublicKey
    );
    export_handler!(
        derive_virtual_account_address::Handler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_deriveVirtualAccountAddress
    );
    export_handler!(
        derive_virtual_identity_address::Handler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_deriveVirtualIdentityAddress
    );

    export_handler!(
        encode_address::Handler as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_encodeAddress
    );
    export_handler!(
        decode_address::Handler as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_decodeAddress
    );

    export_handler!(
        sbor_encode::Handler as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_sborEncode
    );
    export_handler!(
        sbor_decode::Handler as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_sborDecode
    );

    export_handler!(
        known_entity_addresses::Handler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_knownEntityAddresses
    );
    export_handler!(
        statically_validate_transaction::Handler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_staticallyValidateTransaction
    );

    export_handler!(hash::Handler as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_hash);
}
