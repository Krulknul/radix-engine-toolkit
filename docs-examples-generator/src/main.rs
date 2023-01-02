#![macro_use]
extern crate lazy_static;

mod examples;
mod examples_builder;

use std::fs;

use examples_builder::InMemoryExamplesBuilder;
use radix_engine_toolkit_core::requests::*;

fn main() {
    let examples = InMemoryExamplesBuilder::new()
        .add_example::<'_, InformationRequest, InformationResponse>()
        .add_example::<'_, ConvertManifestRequest, ConvertManifestResponse>()
        .add_example::<'_, CompileTransactionIntentRequest, CompileTransactionIntentResponse>()
        .add_example::<'_, DecompileTransactionIntentRequest, DecompileTransactionIntentResponse>()
        .add_example::<'_, CompileSignedTransactionIntentRequest, CompileSignedTransactionIntentResponse>()
        .add_example::<'_, DecompileSignedTransactionIntentRequest, DecompileSignedTransactionIntentResponse>()
        .add_example::<'_, CompileNotarizedTransactionIntentRequest, CompileNotarizedTransactionIntentResponse>()
        .add_example::<'_, DecompileNotarizedTransactionIntentRequest, DecompileNotarizedTransactionIntentResponse>()
        .add_example::<'_, EncodeAddressRequest, EncodeAddressResponse>()
        .add_example::<'_, DecodeAddressRequest, DecodeAddressResponse>()
        .add_example::<'_, SBOREncodeRequest, SBOREncodeResponse>()
        .add_example::<'_, SBORDecodeRequest, SBORDecodeResponse>()
        .add_example::<'_, DeriveVirtualAccountAddressRequest, DeriveVirtualAccountAddressResponse>()
        .add_example::<'_, KnownEntityAddressesRequest, KnownEntityAddressesResponse>()
        .add_example::<'_, StaticallyValidateTransactionRequest, StaticallyValidateTransactionResponse>()
        .build();
    fs::write("./request-examples.md", examples).unwrap();
}