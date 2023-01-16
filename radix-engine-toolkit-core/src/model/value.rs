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

//! This module implements the [Value] struct as well as all of its related methods for conversion
//! and validation.

use itertools::Itertools;
use radix_transaction::manifest::ast::Value as AstValue;
use sbor::value_kind::*;
use sbor::CustomValueKind;
use scrypto::prelude::{
    scrypto_decode, scrypto_encode, Decimal, EcdsaSecp256k1PublicKey, EcdsaSecp256k1Signature,
    EddsaEd25519PublicKey, EddsaEd25519Signature, Hash, ManifestBlobRef, ManifestExpression,
    NonFungibleId, PreciseDecimal, ScryptoCustomValue, ScryptoCustomValueKind, ScryptoValue,
    ScryptoValueKind,
};
use scrypto::runtime::{ManifestBucket, ManifestProof, Own};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, FromInto};

use super::{BucketId, Identifier, NonFungibleAddress, OptionProxy, ProofId, ResultProxy};
use crate::error::Error;
use crate::model::address::*;
use crate::traits::ValidateWithContext;

// ======
// Value
// ======

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrU32 {
    String(String),
    U32(u32),
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
#[serde(tag = "type")]
pub enum Value {
    Bool {
        value: bool,
    },

    U8 {
        #[serde_as(as = "DisplayFromStr")]
        value: u8,
    },
    U16 {
        #[serde_as(as = "DisplayFromStr")]
        value: u16,
    },
    U32 {
        #[serde_as(as = "DisplayFromStr")]
        value: u32,
    },
    U64 {
        #[serde_as(as = "DisplayFromStr")]
        value: u64,
    },
    U128 {
        #[serde_as(as = "DisplayFromStr")]
        value: u128,
    },

    I8 {
        #[serde_as(as = "DisplayFromStr")]
        value: i8,
    },
    I16 {
        #[serde_as(as = "DisplayFromStr")]
        value: i16,
    },
    I32 {
        #[serde_as(as = "DisplayFromStr")]
        value: i32,
    },
    I64 {
        #[serde_as(as = "DisplayFromStr")]
        value: i64,
    },
    I128 {
        #[serde_as(as = "DisplayFromStr")]
        value: i128,
    },

    String {
        value: String,
    },

    Enum {
        variant: u8,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        fields: Option<Vec<Value>>,
    },
    Option {
        #[serde(flatten)]
        #[serde_as(as = "Box<FromInto<OptionProxy<Value>>>")]
        value: Box<Option<Value>>,
    },
    Result {
        #[serde(flatten)]
        #[serde_as(as = "Box<FromInto<ResultProxy<Value, Value>>>")]
        value: Box<Result<Value, Value>>,
    },

    Array {
        element_type: ValueKind,
        elements: Vec<Value>,
    },
    Map {
        key_type: ValueKind,
        value_type: ValueKind,
        elements: Vec<Value>,
    },
    Tuple {
        elements: Vec<Value>,
    },

    // Scrypto Values
    Decimal {
        #[serde_as(as = "DisplayFromStr")]
        value: Decimal,
    },
    PreciseDecimal {
        #[serde_as(as = "DisplayFromStr")]
        value: PreciseDecimal,
    },

    ComponentAddress {
        address: NetworkAwareComponentAddress,
    },
    ResourceAddress {
        address: NetworkAwareResourceAddress,
    },
    PackageAddress {
        address: NetworkAwarePackageAddress,
    },
    SystemAddress {
        address: NetworkAwareSystemAddress,
    },

    Hash {
        #[serde_as(as = "DisplayFromStr")]
        value: Hash,
    },
    EcdsaSecp256k1PublicKey {
        #[serde_as(as = "DisplayFromStr")]
        public_key: EcdsaSecp256k1PublicKey,
    },
    EcdsaSecp256k1Signature {
        #[serde_as(as = "DisplayFromStr")]
        signature: EcdsaSecp256k1Signature,
    },
    EddsaEd25519PublicKey {
        #[serde_as(as = "DisplayFromStr")]
        public_key: EddsaEd25519PublicKey,
    },
    EddsaEd25519Signature {
        #[serde_as(as = "DisplayFromStr")]
        signature: EddsaEd25519Signature,
    },

    Bucket {
        identifier: BucketId,
    },
    Proof {
        identifier: ProofId,
    },
    NonFungibleId {
        #[serde(flatten)]
        #[serde_as(as = "FromInto<crate::model::helper::NonFungibleIdProxy>")]
        value: NonFungibleId,
    },
    NonFungibleAddress {
        #[serde(flatten)]
        address: NonFungibleAddress,
    },

    Blob {
        #[serde_as(as = "FromInto<crate::model::helper::ManifestBlobRefProxy>")]
        hash: ManifestBlobRef,
    },
    Expression {
        #[serde_as(as = "FromInto<crate::model::helper::ExpressionProxy>")]
        value: ManifestExpression,
    },
    Bytes {
        #[serde_as(as = "serde_with::hex::Hex")]
        value: Vec<u8>,
    },

    Own {
        #[serde_as(as = "FromInto<crate::model::helper::OwnProxy>")]
        value: Own,
    },
}

impl Value {
    pub fn kind(&self) -> ValueKind {
        match self {
            Self::Bool { .. } => ValueKind::Bool,

            Self::I8 { .. } => ValueKind::I8,
            Self::I16 { .. } => ValueKind::I16,
            Self::I32 { .. } => ValueKind::I32,
            Self::I64 { .. } => ValueKind::I64,
            Self::I128 { .. } => ValueKind::I128,

            Self::U8 { .. } => ValueKind::U8,
            Self::U16 { .. } => ValueKind::U16,
            Self::U32 { .. } => ValueKind::U32,
            Self::U64 { .. } => ValueKind::U64,
            Self::U128 { .. } => ValueKind::U128,

            Self::String { .. } => ValueKind::String,

            Self::Enum { .. } => ValueKind::Enum,
            Self::Option { .. } => ValueKind::Option,
            Self::Result { .. } => ValueKind::Result,

            Self::Array { .. } => ValueKind::Array,
            Self::Map { .. } => ValueKind::Map,
            Self::Tuple { .. } => ValueKind::Tuple,

            Self::Decimal { .. } => ValueKind::Decimal,
            Self::PreciseDecimal { .. } => ValueKind::PreciseDecimal,

            Self::PackageAddress { .. } => ValueKind::PackageAddress,
            Self::ComponentAddress { .. } => ValueKind::ComponentAddress,
            Self::ResourceAddress { .. } => ValueKind::ResourceAddress,
            Self::SystemAddress { .. } => ValueKind::SystemAddress,

            Self::Hash { .. } => ValueKind::Hash,

            Self::Bucket { .. } => ValueKind::Bucket,
            Self::Proof { .. } => ValueKind::Proof,

            Self::NonFungibleId { .. } => ValueKind::NonFungibleId,
            Self::NonFungibleAddress { .. } => ValueKind::NonFungibleAddress,

            Self::EcdsaSecp256k1PublicKey { .. } => ValueKind::EcdsaSecp256k1PublicKey,
            Self::EcdsaSecp256k1Signature { .. } => ValueKind::EcdsaSecp256k1Signature,
            Self::EddsaEd25519PublicKey { .. } => ValueKind::EddsaEd25519PublicKey,
            Self::EddsaEd25519Signature { .. } => ValueKind::EddsaEd25519Signature,

            Self::Blob { .. } => ValueKind::Blob,
            Self::Expression { .. } => ValueKind::Expression,
            Self::Bytes { .. } => ValueKind::Bytes,
            Self::Own { .. } => ValueKind::Own,
        }
    }

    // ===========
    // Validation
    // ===========

    fn validate_kind(&self, expected_kind: ValueKind) -> Result<(), Error> {
        if self.kind() == expected_kind {
            Ok(())
        } else {
            Err(Error::InvalidType {
                expected_types: vec![expected_kind],
                actual_type: self.kind(),
            })
        }
    }

    fn validate_if_collection(&self, network_id: u8) -> Result<(), Error> {
        match self {
            Self::Array {
                element_type,
                elements,
            } => {
                elements
                    .iter()
                    .map(|item| item.validate((network_id, Some(*element_type))))
                    .collect::<Result<Vec<()>, _>>()?;
                Ok(())
            }
            Self::Tuple { elements } => {
                elements
                    .iter()
                    .map(|item| item.validate((network_id, None)))
                    .collect::<Result<Vec<()>, _>>()?;
                Ok(())
            }
            // Not a collection. No validation required.
            _ => Ok(()),
        }
    }

    fn validate_address_network_id(&self, expected_network_id: u8) -> Result<(), Error> {
        let network_id = match self {
            Self::ComponentAddress { address } => address.network_id,
            Self::ResourceAddress { address } => address.network_id,
            Self::PackageAddress { address } => address.network_id,
            Self::SystemAddress { address } => address.network_id,
            _ => return Ok(()),
        };
        if network_id == expected_network_id {
            Ok(())
        } else {
            Err(Error::NetworkMismatchError {
                expected: expected_network_id,
                found: network_id,
            })
        }
    }

    // ============
    // Conversions
    // ============

    pub fn from_ast_value(ast_value: &AstValue, bech32_coder: &Bech32Coder) -> Result<Self, Error> {
        let value = match ast_value {
            AstValue::Bool(value) => Self::Bool { value: *value },

            AstValue::I8(value) => Self::I8 { value: *value },
            AstValue::I16(value) => Self::I16 { value: *value },
            AstValue::I32(value) => Self::I32 { value: *value },
            AstValue::I64(value) => Self::I64 { value: *value },
            AstValue::I128(value) => Self::I128 { value: *value },

            AstValue::U8(value) => Self::U8 { value: *value },
            AstValue::U16(value) => Self::U16 { value: *value },
            AstValue::U32(value) => Self::U32 { value: *value },
            AstValue::U64(value) => Self::U64 { value: *value },
            AstValue::U128(value) => Self::U128 { value: *value },

            AstValue::String(value) => Self::String {
                value: value.clone(),
            },

            AstValue::Enum(variant, fields) => match (variant, fields.len()) {
                (0, 1) => Self::Option {
                    value: Box::new(Some(Self::from_ast_value(&fields[0], bech32_coder)?)),
                },
                (1, 0) => Self::Option {
                    value: Box::new(None),
                },
                (0, 1) => Self::Result {
                    value: Box::new(Ok(Self::from_ast_value(&fields[0], bech32_coder)?)),
                },
                (1, 1) => Self::Result {
                    value: Box::new(Err(Self::from_ast_value(&fields[0], bech32_coder)?)),
                },
                _ => Self::Enum {
                    variant: variant.clone(),
                    fields: {
                        let fields = fields
                            .iter()
                            .map(|v| Self::from_ast_value(v, bech32_coder))
                            .collect::<Result<Vec<Value>, _>>()?;
                        match fields.len() {
                            0 => None,
                            _ => Some(fields),
                        }
                    },
                },
            },
            AstValue::Some(value) => Self::Option {
                value: Box::new(Some(Self::from_ast_value(value, bech32_coder)?)),
            },
            AstValue::None => Self::Option {
                value: Box::new(None),
            },
            AstValue::Ok(value) => Self::Result {
                value: Box::new(Ok(Self::from_ast_value(value, bech32_coder)?)),
            },
            AstValue::Err(value) => Self::Result {
                value: Box::new(Err(Self::from_ast_value(value, bech32_coder)?)),
            },

            AstValue::Array(ast_type, elements) => Self::Array {
                element_type: (*ast_type).into(),
                elements: elements
                    .iter()
                    .map(|v| Self::from_ast_value(v, bech32_coder))
                    .collect::<Result<Vec<Value>, _>>()?,
            },
            AstValue::Map(key_type, value_type, elements) => Self::Map {
                key_type: (*key_type).into(),
                value_type: (*value_type).into(),
                elements: elements
                    .iter()
                    .map(|v| Self::from_ast_value(v, bech32_coder))
                    .collect::<Result<Vec<Value>, _>>()?,
            },
            AstValue::Tuple(elements) => Self::Tuple {
                elements: elements
                    .iter()
                    .map(|v| Self::from_ast_value(v, bech32_coder))
                    .collect::<Result<Vec<Value>, _>>()?,
            },

            AstValue::Decimal(value) => {
                if let AstValue::String(value) = &**value {
                    Self::Decimal {
                        value: value.parse()?,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::Decimal,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.type_id().into(),
                    })?
                }
            }
            AstValue::PreciseDecimal(value) => {
                if let AstValue::String(value) = &**value {
                    Self::PreciseDecimal {
                        value: value.parse()?,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::PreciseDecimal,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.type_id().into(),
                    })?
                }
            }

            AstValue::PackageAddress(value) => {
                if let AstValue::String(value) = &**value {
                    Self::PackageAddress {
                        address: NetworkAwarePackageAddress {
                            network_id: bech32_coder.network_id(),
                            address: bech32_coder
                                .decoder
                                .validate_and_decode_package_address(value)?,
                        },
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::PackageAddress,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.type_id().into(),
                    })?
                }
            }
            AstValue::ComponentAddress(value) => {
                if let AstValue::String(value) = &**value {
                    Self::ComponentAddress {
                        address: NetworkAwareComponentAddress {
                            network_id: bech32_coder.network_id(),
                            address: bech32_coder
                                .decoder
                                .validate_and_decode_component_address(value)?,
                        },
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::ComponentAddress,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.type_id().into(),
                    })?
                }
            }
            AstValue::ResourceAddress(value) => {
                if let AstValue::String(value) = &**value {
                    Self::ResourceAddress {
                        address: NetworkAwareResourceAddress {
                            network_id: bech32_coder.network_id(),
                            address: bech32_coder
                                .decoder
                                .validate_and_decode_resource_address(value)?,
                        },
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::ResourceAddress,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.type_id().into(),
                    })?
                }
            }
            AstValue::SystemAddress(value) => {
                if let AstValue::String(value) = &**value {
                    Self::SystemAddress {
                        address: NetworkAwareSystemAddress {
                            network_id: bech32_coder.network_id(),
                            address: bech32_coder
                                .decoder
                                .validate_and_decode_system_address(value)?,
                        },
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::SystemAddress,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.type_id().into(),
                    })?
                }
            }

            AstValue::Hash(value) => {
                if let AstValue::String(value) = &**value {
                    Self::Hash {
                        value: value.parse()?,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::Hash,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.type_id().into(),
                    })?
                }
            }

            AstValue::Bucket(value) => {
                if let AstValue::U32(value) = &**value {
                    Self::Bucket {
                        identifier: Identifier::U32(*value).into(),
                    }
                } else if let AstValue::String(value) = &**value {
                    Self::Bucket {
                        identifier: Identifier::String(value.clone()).into(),
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::Bucket,
                        allowed_children_kinds: vec![ValueKind::U32, ValueKind::String],
                        found_child_kind: value.type_id().into(),
                    })?
                }
            }
            AstValue::Proof(value) => {
                if let AstValue::U32(value) = &**value {
                    Self::Proof {
                        identifier: Identifier::U32(*value).into(),
                    }
                } else if let AstValue::String(value) = &**value {
                    Self::Proof {
                        identifier: Identifier::String(value.clone()).into(),
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::Proof,
                        allowed_children_kinds: vec![ValueKind::U32, ValueKind::String],
                        found_child_kind: value.type_id().into(),
                    })?
                }
            }

            AstValue::NonFungibleId(value) => Self::NonFungibleId {
                value: match &**value {
                    AstValue::U64(value) => NonFungibleId::Number(*value),
                    AstValue::U128(value) => NonFungibleId::UUID(*value),
                    AstValue::String(value) => NonFungibleId::String(value.clone()),
                    AstValue::Bytes(value) => {
                        if let AstValue::String(value) = &**value {
                            NonFungibleId::Bytes(hex::decode(value)?)
                        } else {
                            Err(Error::UnexpectedContents {
                                kind_being_parsed: ValueKind::NonFungibleId,
                                allowed_children_kinds: vec![ValueKind::String],
                                found_child_kind: value.type_id().into(),
                            })?
                        }
                    }
                    _ => Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::NonFungibleId,
                        allowed_children_kinds: vec![
                            ValueKind::U32,
                            ValueKind::U64,
                            ValueKind::U128,
                            ValueKind::String,
                            ValueKind::Bytes,
                        ],
                        found_child_kind: value.type_id().into(),
                    })?,
                },
            },
            AstValue::NonFungibleAddress(resource_address, non_fungible_id) => {
                let resource_address = if let AstValue::String(address_string) = &**resource_address
                {
                    let address = bech32_coder
                        .decoder
                        .validate_and_decode_resource_address(address_string)?;
                    NetworkAwareResourceAddress {
                        network_id: bech32_coder.network_id(),
                        address,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::NonFungibleAddress,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: resource_address.type_id().into(),
                    })?
                };

                // TODO: de-duplicate. Refactor out
                let non_fungible_id = match &**non_fungible_id {
                    AstValue::U64(value) => NonFungibleId::Number(*value),
                    AstValue::U128(value) => NonFungibleId::UUID(*value),
                    AstValue::String(value) => NonFungibleId::String(value.clone()),
                    AstValue::Bytes(value) => {
                        if let AstValue::String(value) = &**value {
                            NonFungibleId::Bytes(hex::decode(value)?)
                        } else {
                            Err(Error::UnexpectedContents {
                                kind_being_parsed: ValueKind::NonFungibleAddress,
                                allowed_children_kinds: vec![ValueKind::String],
                                found_child_kind: value.type_id().into(),
                            })?
                        }
                    }
                    value => Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::NonFungibleAddress,
                        allowed_children_kinds: vec![
                            ValueKind::U32,
                            ValueKind::U64,
                            ValueKind::U128,
                            ValueKind::String,
                            ValueKind::Bytes,
                        ],
                        found_child_kind: value.type_id().into(),
                    })?,
                };

                let non_fungible_address = NonFungibleAddress {
                    resource_address,
                    non_fungible_id,
                };
                Value::NonFungibleAddress {
                    address: non_fungible_address,
                }
            }

            AstValue::Blob(value) => {
                if let AstValue::String(value) = &**value {
                    Self::Blob {
                        hash: ManifestBlobRef::try_from(hex::decode(value)?.as_slice())?,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::Blob,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.type_id().into(),
                    })?
                }
            }
            AstValue::Expression(value) => {
                if let AstValue::String(value) = &**value {
                    Self::Expression {
                        value: match value.as_str() {
                            "ENTIRE_WORKTOP" => ManifestExpression::EntireWorktop,
                            "ENTIRE_AUTH_ZONE" => ManifestExpression::EntireAuthZone,
                            _ => todo!(), // TODO: Remove
                        },
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::Expression,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.type_id().into(),
                    })?
                }
            }

            AstValue::EcdsaSecp256k1PublicKey(value) => {
                if let AstValue::String(value) = &**value {
                    Self::EcdsaSecp256k1PublicKey {
                        public_key: value.parse()?,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::EcdsaSecp256k1PublicKey,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.type_id().into(),
                    })?
                }
            }
            AstValue::EcdsaSecp256k1Signature(value) => {
                if let AstValue::String(value) = &**value {
                    Self::EcdsaSecp256k1Signature {
                        signature: value.parse()?,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::EcdsaSecp256k1Signature,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.type_id().into(),
                    })?
                }
            }
            AstValue::EddsaEd25519PublicKey(value) => {
                if let AstValue::String(value) = &**value {
                    Self::EddsaEd25519PublicKey {
                        public_key: value.parse()?,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::EddsaEd25519PublicKey,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.type_id().into(),
                    })?
                }
            }
            AstValue::EddsaEd25519Signature(value) => {
                if let AstValue::String(value) = &**value {
                    Self::EddsaEd25519Signature {
                        signature: value.parse()?,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::EddsaEd25519Signature,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.type_id().into(),
                    })?
                }
            }
            AstValue::Bytes(value) => {
                if let AstValue::String(value) = &**value {
                    Self::Bytes {
                        value: hex::decode(value)?,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::Bytes,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.type_id().into(),
                    })?
                }
            }
            AstValue::Own(..) => todo!(), // TODO: TODO
        };
        Ok(value)
    }

    pub fn to_ast_value(&self, bech32_coder: &Bech32Coder) -> Result<AstValue, Error> {
        let ast_value = match self {
            Value::Bool { value } => AstValue::Bool(*value),

            Value::I8 { value } => AstValue::I8(*value),
            Value::I16 { value } => AstValue::I16(*value),
            Value::I32 { value } => AstValue::I32(*value),
            Value::I64 { value } => AstValue::I64(*value),
            Value::I128 { value } => AstValue::I128(*value),

            Value::U8 { value } => AstValue::U8(*value),
            Value::U16 { value } => AstValue::U16(*value),
            Value::U32 { value } => AstValue::U32(*value),
            Value::U64 { value } => AstValue::U64(*value),
            Value::U128 { value } => AstValue::U128(*value),

            Value::String { value } => AstValue::String(value.clone()),

            Value::Enum { variant, fields } => AstValue::Enum(
                variant.clone(),
                fields
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|v| v.to_ast_value(bech32_coder))
                    .collect::<Result<Vec<AstValue>, _>>()?,
            ),
            Value::Option { value } => match &**value {
                Some(value) => AstValue::Some(Box::new(value.to_ast_value(bech32_coder)?)),
                None => AstValue::None,
            },
            Value::Result { value } => match &**value {
                Ok(value) => AstValue::Ok(Box::new(value.to_ast_value(bech32_coder)?)),
                Err(value) => AstValue::Err(Box::new(value.to_ast_value(bech32_coder)?)),
            },

            Value::Array {
                element_type,
                elements,
            } => AstValue::Array(
                (*element_type).into(),
                elements
                    .iter()
                    .map(|id| id.to_ast_value(bech32_coder))
                    .collect::<Result<Vec<AstValue>, Error>>()?,
            ),
            Value::Tuple { elements } => AstValue::Tuple(
                elements
                    .iter()
                    .map(|v| v.to_ast_value(bech32_coder))
                    .collect::<Result<Vec<AstValue>, _>>()?,
            ),

            Value::Decimal { value } => {
                AstValue::Decimal(Box::new(AstValue::String(value.to_string())))
            }
            Value::PreciseDecimal { value } => {
                AstValue::PreciseDecimal(Box::new(AstValue::String(value.to_string())))
            }

            Value::PackageAddress { address: value } => {
                AstValue::PackageAddress(Box::new(AstValue::String(
                    bech32_coder
                        .encoder
                        .encode_package_address_to_string(&value.address),
                )))
            }
            Value::ComponentAddress { address: value } => {
                AstValue::ComponentAddress(Box::new(AstValue::String(
                    bech32_coder
                        .encoder
                        .encode_component_address_to_string(&value.address),
                )))
            }
            Value::ResourceAddress { address: value } => {
                AstValue::ResourceAddress(Box::new(AstValue::String(
                    bech32_coder
                        .encoder
                        .encode_resource_address_to_string(&value.address),
                )))
            }
            Value::SystemAddress { address: value } => {
                AstValue::SystemAddress(Box::new(AstValue::String(
                    bech32_coder
                        .encoder
                        .encode_system_address_to_string(&value.address),
                )))
            }

            Value::Hash { value } => AstValue::Hash(Box::new(AstValue::String(value.to_string()))),
            Value::Bucket { identifier } => AstValue::Bucket(Box::new(match identifier.0 {
                Identifier::String(ref string) => AstValue::String(string.clone()),
                Identifier::U32(number) => AstValue::U32(number),
            })),
            Value::Proof { identifier } => AstValue::Proof(Box::new(match identifier.0 {
                Identifier::String(ref string) => AstValue::String(string.clone()),
                Identifier::U32(number) => AstValue::U32(number),
            })),

            Value::NonFungibleId { value } => AstValue::NonFungibleId(Box::new(match value {
                NonFungibleId::Number(value) => AstValue::U64(*value),
                NonFungibleId::UUID(value) => AstValue::U128(*value),
                NonFungibleId::String(ref value) => AstValue::String(value.clone()),
                NonFungibleId::Bytes(ref value) => {
                    AstValue::Bytes(Box::new(AstValue::String(hex::encode(value))))
                }
            })),
            Value::NonFungibleAddress { address } => {
                let resource_address_string = address.resource_address.to_string();
                let resource_address = AstValue::String(resource_address_string);

                let non_fungible_id = match address.non_fungible_id {
                    NonFungibleId::Number(value) => AstValue::U64(value),
                    NonFungibleId::UUID(value) => AstValue::U128(value),
                    NonFungibleId::String(ref value) => AstValue::String(value.clone()),
                    NonFungibleId::Bytes(ref value) => {
                        AstValue::Bytes(Box::new(AstValue::String(hex::encode(value))))
                    }
                };

                AstValue::NonFungibleAddress(Box::new(resource_address), Box::new(non_fungible_id))
            }

            Value::Blob { hash } => AstValue::Blob(Box::new(AstValue::String(hash.0.to_string()))),
            Value::Expression { value } => {
                AstValue::Expression(Box::new(AstValue::String(match value {
                    ManifestExpression::EntireWorktop => "ENTIRE_WORKTOP".into(),
                    ManifestExpression::EntireAuthZone => "ENTIRE_AUTH_ZONE".into(),
                })))
            }

            Value::EcdsaSecp256k1PublicKey { public_key } => AstValue::EcdsaSecp256k1PublicKey(
                Box::new(AstValue::String(public_key.to_string())),
            ),
            Value::EcdsaSecp256k1Signature { signature } => {
                AstValue::EcdsaSecp256k1Signature(Box::new(AstValue::String(signature.to_string())))
            }

            Value::EddsaEd25519PublicKey { public_key } => {
                AstValue::EddsaEd25519PublicKey(Box::new(AstValue::String(public_key.to_string())))
            }
            Value::EddsaEd25519Signature { signature } => {
                AstValue::EddsaEd25519Signature(Box::new(AstValue::String(signature.to_string())))
            }
            Value::Bytes { value } => {
                AstValue::Bytes(Box::new(AstValue::String(hex::encode(value))))
            }
            Value::Map {
                key_type,
                value_type,
                elements,
            } => AstValue::Map(
                (*key_type).into(),
                (*value_type).into(),
                elements
                    .iter()
                    .map(|id| id.to_ast_value(bech32_coder))
                    .collect::<Result<Vec<AstValue>, Error>>()?,
            ),
            Value::Own { .. } => {
                todo!() // TODO: TODO
            }
        };
        Ok(ast_value)
    }

    pub fn to_scrypto_value(&self) -> Result<ScryptoValue, Error> {
        let scrypto_value = match self {
            Value::Bool { value } => ScryptoValue::Bool { value: *value },

            Value::U8 { value } => ScryptoValue::U8 { value: *value },
            Value::U16 { value } => ScryptoValue::U16 { value: *value },
            Value::U32 { value } => ScryptoValue::U32 { value: *value },
            Value::U64 { value } => ScryptoValue::U64 { value: *value },
            Value::U128 { value } => ScryptoValue::U128 { value: *value },

            Value::I8 { value } => ScryptoValue::I8 { value: *value },
            Value::I16 { value } => ScryptoValue::I16 { value: *value },
            Value::I32 { value } => ScryptoValue::I32 { value: *value },
            Value::I64 { value } => ScryptoValue::I64 { value: *value },
            Value::I128 { value } => ScryptoValue::I128 { value: *value },

            Value::String { value } => ScryptoValue::String {
                value: value.clone(),
            },
            Value::Enum { variant, fields } => ScryptoValue::Enum {
                discriminator: variant.clone(),
                fields: fields
                    .clone()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|x| x.to_scrypto_value())
                    .collect::<Result<Vec<_>, _>>()?,
            },
            Value::Option { value } => match &**value {
                Some(value) => ScryptoValue::Enum {
                    discriminator: 0,
                    fields: vec![value.to_scrypto_value()?],
                },
                None => ScryptoValue::Enum {
                    discriminator: 1,
                    fields: Vec::new(),
                },
            },
            Value::Result { value } => match &**value {
                Ok(value) => ScryptoValue::Enum {
                    discriminator: 0,
                    fields: vec![value.to_scrypto_value()?],
                },
                Err(value) => ScryptoValue::Enum {
                    discriminator: 1,
                    fields: vec![value.to_scrypto_value()?],
                },
            },
            Value::Array {
                element_type,
                elements,
            } => ScryptoValue::Array {
                element_value_kind: (*element_type).into(),
                elements: elements
                    .clone()
                    .into_iter()
                    .map(|x| x.to_scrypto_value())
                    .collect::<Result<Vec<_>, _>>()?,
            },
            Value::Map {
                key_type,
                value_type,
                elements,
            } => {
                let keys = elements
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i % 2 == 0)
                    .map(|(_, v)| v.to_scrypto_value())
                    .collect::<Result<Vec<_>, _>>()?;
                let values = elements
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i % 2 != 0)
                    .map(|(_, v)| v.to_scrypto_value())
                    .collect::<Result<Vec<_>, _>>()?;

                ScryptoValue::Map {
                    key_value_kind: (*key_type).into(),
                    value_value_kind: (*value_type).into(),
                    entries: keys.into_iter().zip(values).collect(),
                }
            }
            Value::Tuple { elements } => ScryptoValue::Tuple {
                fields: elements
                    .clone()
                    .into_iter()
                    .map(|x| x.to_scrypto_value())
                    .collect::<Result<Vec<_>, _>>()?,
            },

            Value::Decimal { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::Decimal(*value),
            },
            Value::PreciseDecimal { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::PreciseDecimal(*value),
            },
            Value::ComponentAddress { address } => ScryptoValue::Custom {
                value: ScryptoCustomValue::ComponentAddress(address.address),
            },
            Value::PackageAddress { address } => ScryptoValue::Custom {
                value: ScryptoCustomValue::PackageAddress(address.address),
            },
            Value::ResourceAddress { address } => ScryptoValue::Custom {
                value: ScryptoCustomValue::ResourceAddress(address.address),
            },
            Value::SystemAddress { address } => ScryptoValue::Custom {
                value: ScryptoCustomValue::SystemAddress(address.address),
            },

            Value::Hash { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::Hash(*value),
            },

            Value::EcdsaSecp256k1PublicKey { public_key } => ScryptoValue::Custom {
                value: ScryptoCustomValue::EcdsaSecp256k1PublicKey(*public_key),
            },
            Value::EddsaEd25519PublicKey { public_key } => ScryptoValue::Custom {
                value: ScryptoCustomValue::EddsaEd25519PublicKey(*public_key),
            },

            Value::EcdsaSecp256k1Signature { signature } => ScryptoValue::Custom {
                value: ScryptoCustomValue::EcdsaSecp256k1Signature(*signature),
            },
            Value::EddsaEd25519Signature { signature } => ScryptoValue::Custom {
                value: ScryptoCustomValue::EddsaEd25519Signature(*signature),
            },

            Value::Bucket { identifier } => ScryptoValue::Custom {
                value: match identifier.0 {
                    Identifier::U32(numeric_identifier) => {
                        ScryptoCustomValue::Bucket(ManifestBucket(numeric_identifier))
                    }
                    Identifier::String(_) => {
                        return Err(Error::SborEncodeError(
                            "Unable to encode a Bucket with a string identifier".into(),
                        ));
                    }
                },
            },
            Value::Proof { identifier } => ScryptoValue::Custom {
                value: match identifier.0 {
                    Identifier::U32(numeric_identifier) => {
                        ScryptoCustomValue::Proof(ManifestProof(numeric_identifier))
                    }
                    Identifier::String(_) => {
                        return Err(Error::SborEncodeError(
                            "Unable to encode a Proof with a string identifier".into(),
                        ));
                    }
                },
            },

            Value::NonFungibleId { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::NonFungibleId(value.clone()),
            },
            Value::NonFungibleAddress { address } => ScryptoValue::Tuple {
                fields: vec![
                    Value::ResourceAddress {
                        address: address.resource_address.clone(),
                    }
                    .to_scrypto_value()?,
                    Value::NonFungibleId {
                        value: address.non_fungible_id.clone(),
                    }
                    .to_scrypto_value()?,
                ],
            },

            Value::Blob { hash } => ScryptoValue::Custom {
                value: ScryptoCustomValue::Blob(hash.clone()),
            },
            Value::Expression { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::Expression(value.clone()),
            },
            Value::Bytes { value } => ScryptoValue::Array {
                element_value_kind: ScryptoValueKind::U8,
                elements: value
                    .clone()
                    .into_iter()
                    .map(|value| ScryptoValue::U8 { value })
                    .collect(),
            },

            Value::Own { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::Own(value.clone()),
            },
        };
        Ok(scrypto_value)
    }

    pub fn from_scrypto_value(scrypto_value: &ScryptoValue, network_id: u8) -> Self {
        match scrypto_value {
            ScryptoValue::Bool { value } => Value::Bool { value: *value },

            ScryptoValue::U8 { value } => Value::U8 { value: *value },
            ScryptoValue::U16 { value } => Value::U16 { value: *value },
            ScryptoValue::U32 { value } => Value::U32 { value: *value },
            ScryptoValue::U64 { value } => Value::U64 { value: *value },
            ScryptoValue::U128 { value } => Value::U128 { value: *value },

            ScryptoValue::I8 { value } => Value::I8 { value: *value },
            ScryptoValue::I16 { value } => Value::I16 { value: *value },
            ScryptoValue::I32 { value } => Value::I32 { value: *value },
            ScryptoValue::I64 { value } => Value::I64 { value: *value },
            ScryptoValue::I128 { value } => Value::I128 { value: *value },

            ScryptoValue::String { value } => Value::String {
                value: value.clone(),
            },

            ScryptoValue::Enum {
                discriminator,
                fields,
            } => match (discriminator, fields.len()) {
                (0, 1) => Value::Option {
                    value: Box::new(Some(Self::from_scrypto_value(&fields[0], network_id))),
                },
                (1, 0) => Value::Option {
                    value: Box::new(None),
                },
                (0, 1) => Value::Result {
                    value: Box::new(Ok(Self::from_scrypto_value(&fields[0], network_id))),
                },
                (1, 1) => Value::Result {
                    value: Box::new(Err(Self::from_scrypto_value(&fields[0], network_id))),
                },
                _ => Value::Enum {
                    variant: discriminator.clone(),
                    fields: if fields.is_empty() {
                        None
                    } else {
                        Some(
                            fields
                                .clone()
                                .into_iter()
                                .map(|x| Self::from_scrypto_value(&x, network_id))
                                .collect(),
                        )
                    },
                },
            },
            ScryptoValue::Array {
                element_value_kind,
                elements,
            } => Value::Array {
                element_type: (*element_value_kind).into(),
                elements: elements
                    .clone()
                    .into_iter()
                    .map(|x| Self::from_scrypto_value(&x, network_id))
                    .collect(),
            },
            ScryptoValue::Map {
                key_value_kind,
                value_value_kind,
                entries,
            } => Value::Map {
                key_type: (*key_value_kind).into(),
                value_type: (*value_value_kind).into(),
                elements: entries
                    .clone()
                    .into_iter()
                    .flat_map(|(x, y)| [x, y])
                    .map(|x| Self::from_scrypto_value(&x, network_id))
                    .collect(),
            },
            ScryptoValue::Tuple { fields } => Value::Tuple {
                elements: fields
                    .clone()
                    .into_iter()
                    .map(|x| Self::from_scrypto_value(&x, network_id))
                    .collect(),
            },

            ScryptoValue::Custom { value } => match value {
                ScryptoCustomValue::PackageAddress(address) => Value::PackageAddress {
                    address: NetworkAwarePackageAddress {
                        network_id,
                        address: *address,
                    },
                },
                ScryptoCustomValue::ComponentAddress(address) => Value::ComponentAddress {
                    address: NetworkAwareComponentAddress {
                        network_id,
                        address: *address,
                    },
                },
                ScryptoCustomValue::ResourceAddress(address) => Value::ResourceAddress {
                    address: NetworkAwareResourceAddress {
                        network_id,
                        address: *address,
                    },
                },
                ScryptoCustomValue::SystemAddress(address) => Value::SystemAddress {
                    address: NetworkAwareSystemAddress {
                        network_id,
                        address: *address,
                    },
                },

                ScryptoCustomValue::Bucket(identifier) => Value::Bucket {
                    identifier: Identifier::U32(identifier.0).into(),
                },
                ScryptoCustomValue::Proof(identifier) => Value::Proof {
                    identifier: Identifier::U32(identifier.0).into(),
                },

                ScryptoCustomValue::Expression(value) => Value::Expression {
                    value: value.clone(),
                },
                ScryptoCustomValue::Blob(value) => Value::Blob {
                    hash: value.clone(),
                },
                ScryptoCustomValue::Hash(value) => Value::Hash { value: *value },

                ScryptoCustomValue::EcdsaSecp256k1PublicKey(value) => {
                    Value::EcdsaSecp256k1PublicKey { public_key: *value }
                }
                ScryptoCustomValue::EddsaEd25519PublicKey(value) => {
                    Value::EddsaEd25519PublicKey { public_key: *value }
                }
                ScryptoCustomValue::EcdsaSecp256k1Signature(value) => {
                    Value::EcdsaSecp256k1Signature { signature: *value }
                }
                ScryptoCustomValue::EddsaEd25519Signature(value) => {
                    Value::EddsaEd25519Signature { signature: *value }
                }

                ScryptoCustomValue::Decimal(value) => Value::Decimal { value: *value },
                ScryptoCustomValue::PreciseDecimal(value) => {
                    Value::PreciseDecimal { value: *value }
                }

                ScryptoCustomValue::NonFungibleId(value) => Value::NonFungibleId {
                    value: value.clone(),
                },
                ScryptoCustomValue::Own(value) => Value::Own {
                    value: value.clone(),
                },
            },
        }
    }

    // ===========================
    // SBOR Encoding and Decoding
    // ===========================

    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        let scrypto_value = self.to_scrypto_value()?;
        scrypto_encode(&scrypto_value).map_err(|err| err.into())
    }

    pub fn decode(bytes: &[u8], network_id: u8) -> Result<Self, Error> {
        let scrypto_value = scrypto_decode::<ScryptoValue>(bytes)?;
        Ok(Self::from_scrypto_value(&scrypto_value, network_id))
    }
}

// ===========
// Validation
// ===========

impl ValidateWithContext<(u8, Option<ValueKind>)> for Value {
    fn validate(&self, (network_id, expected_kind): (u8, Option<ValueKind>)) -> Result<(), Error> {
        self.validate_if_collection(network_id)?;
        self.validate_address_network_id(network_id)?;
        if let Some(expected_kind) = expected_kind {
            self.validate_kind(expected_kind)?;
        };
        Ok(())
    }
}

// ==========
// ValueKind
// ==========

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValueKind {
    Bool,

    I8,
    I16,
    I32,
    I64,
    I128,

    U8,
    U16,
    U32,
    U64,
    U128,

    String,

    Enum,
    Option,
    Result,

    Array,
    Map,
    Tuple,

    Decimal,
    PreciseDecimal,

    PackageAddress,
    ComponentAddress,
    ResourceAddress,
    SystemAddress,

    Hash,

    Bucket,
    Proof,

    NonFungibleId,
    NonFungibleAddress,

    EcdsaSecp256k1PublicKey,
    EcdsaSecp256k1Signature,
    EddsaEd25519PublicKey,
    EddsaEd25519Signature,

    Blob,
    Expression,
    Bytes,
    Own,
}

impl ValueKind {
    pub fn type_id(&self) -> u8 {
        match self {
            Self::Bool => ScryptoValueKind::Bool.as_u8(),

            Self::U8 => ScryptoValueKind::U8.as_u8(),
            Self::U16 => ScryptoValueKind::U16.as_u8(),
            Self::U32 => ScryptoValueKind::U32.as_u8(),
            Self::U64 => ScryptoValueKind::U64.as_u8(),
            Self::U128 => ScryptoValueKind::U128.as_u8(),

            Self::I8 => ScryptoValueKind::I8.as_u8(),
            Self::I16 => ScryptoValueKind::I16.as_u8(),
            Self::I32 => ScryptoValueKind::I32.as_u8(),
            Self::I64 => ScryptoValueKind::I64.as_u8(),
            Self::I128 => ScryptoValueKind::I128.as_u8(),

            Self::String => ScryptoValueKind::String.as_u8(),

            Self::Enum => ScryptoValueKind::Enum.as_u8(),
            Self::Option => ScryptoValueKind::Enum.as_u8(),
            Self::Result => ScryptoValueKind::Enum.as_u8(),

            Self::Array => ScryptoValueKind::Array.as_u8(),
            Self::Map => ScryptoValueKind::Map.as_u8(),
            Self::Bytes => ScryptoValueKind::Array.as_u8(),
            Self::Tuple => ScryptoValueKind::Tuple.as_u8(),

            Self::Decimal => ScryptoCustomValueKind::Decimal.as_u8(),
            Self::PreciseDecimal => ScryptoCustomValueKind::PreciseDecimal.as_u8(),

            Self::PackageAddress => ScryptoCustomValueKind::PackageAddress.as_u8(),
            Self::ResourceAddress => ScryptoCustomValueKind::ResourceAddress.as_u8(),
            Self::ComponentAddress => ScryptoCustomValueKind::ComponentAddress.as_u8(),
            Self::SystemAddress => ScryptoCustomValueKind::SystemAddress.as_u8(),

            Self::Hash => ScryptoCustomValueKind::Hash.as_u8(),

            Self::Bucket => ScryptoCustomValueKind::Bucket.as_u8(),
            Self::Proof => ScryptoCustomValueKind::Proof.as_u8(),

            Self::NonFungibleId => ScryptoCustomValueKind::NonFungibleId.as_u8(),
            Self::NonFungibleAddress => ScryptoValueKind::Tuple.as_u8(),

            Self::EcdsaSecp256k1PublicKey => {
                ScryptoCustomValueKind::EcdsaSecp256k1PublicKey.as_u8()
            }
            Self::EcdsaSecp256k1Signature => {
                ScryptoCustomValueKind::EcdsaSecp256k1Signature.as_u8()
            }
            Self::EddsaEd25519PublicKey => ScryptoCustomValueKind::EddsaEd25519PublicKey.as_u8(),
            Self::EddsaEd25519Signature => ScryptoCustomValueKind::EddsaEd25519Signature.as_u8(),

            Self::Blob => ScryptoCustomValueKind::Blob.as_u8(),
            Self::Expression => ScryptoCustomValueKind::Expression.as_u8(),
            Self::Own => ScryptoCustomValueKind::Own.as_u8(),
        }
    }

    pub fn from_type_id(type_id: u8) -> Result<Self, Error> {
        let value_kind = match type_id {
            VALUE_KIND_BOOL => Self::Bool,

            VALUE_KIND_U8 => Self::U8,
            VALUE_KIND_U16 => Self::U16,
            VALUE_KIND_U32 => Self::U32,
            VALUE_KIND_U64 => Self::U64,
            VALUE_KIND_U128 => Self::U128,

            VALUE_KIND_I8 => Self::I8,
            VALUE_KIND_I16 => Self::I16,
            VALUE_KIND_I32 => Self::I32,
            VALUE_KIND_I64 => Self::I64,
            VALUE_KIND_I128 => Self::I128,

            VALUE_KIND_STRING => Self::String,

            VALUE_KIND_ENUM => Self::Enum,

            VALUE_KIND_ARRAY => Self::Array,
            VALUE_KIND_TUPLE => Self::Tuple,

            type_id => match ScryptoCustomValueKind::from_u8(type_id) {
                Some(scrypto_type) => match scrypto_type {
                    ScryptoCustomValueKind::Decimal => Self::Decimal,
                    ScryptoCustomValueKind::PreciseDecimal => Self::PreciseDecimal,
                    ScryptoCustomValueKind::PackageAddress => Self::PackageAddress,
                    ScryptoCustomValueKind::ResourceAddress => Self::ResourceAddress,
                    ScryptoCustomValueKind::ComponentAddress => Self::ComponentAddress,
                    ScryptoCustomValueKind::SystemAddress => Self::SystemAddress,
                    ScryptoCustomValueKind::Hash => Self::Hash,
                    ScryptoCustomValueKind::Bucket => Self::Bucket,
                    ScryptoCustomValueKind::Proof => Self::Proof,
                    ScryptoCustomValueKind::NonFungibleId => Self::NonFungibleId,
                    ScryptoCustomValueKind::EcdsaSecp256k1PublicKey => {
                        Self::EcdsaSecp256k1PublicKey
                    }
                    ScryptoCustomValueKind::EcdsaSecp256k1Signature => {
                        Self::EcdsaSecp256k1Signature
                    }
                    ScryptoCustomValueKind::EddsaEd25519PublicKey => Self::EddsaEd25519PublicKey,
                    ScryptoCustomValueKind::EddsaEd25519Signature => Self::EddsaEd25519Signature,
                    ScryptoCustomValueKind::Blob => Self::Blob,
                    ScryptoCustomValueKind::Expression => Self::Expression,
                    ScryptoCustomValueKind::Own => Self::Own,
                },
                None => return Err(Error::UnknownTypeId { type_id }),
            },
        };
        Ok(value_kind)
    }
}

impl From<ValueKind> for radix_transaction::manifest::ast::Type {
    fn from(value: ValueKind) -> radix_transaction::manifest::ast::Type {
        match value {
            ValueKind::Bool => radix_transaction::manifest::ast::Type::Bool,
            ValueKind::I8 => radix_transaction::manifest::ast::Type::I8,
            ValueKind::I16 => radix_transaction::manifest::ast::Type::I16,
            ValueKind::I32 => radix_transaction::manifest::ast::Type::I32,
            ValueKind::I64 => radix_transaction::manifest::ast::Type::I64,
            ValueKind::I128 => radix_transaction::manifest::ast::Type::I128,

            ValueKind::U8 => radix_transaction::manifest::ast::Type::U8,
            ValueKind::U16 => radix_transaction::manifest::ast::Type::U16,
            ValueKind::U32 => radix_transaction::manifest::ast::Type::U32,
            ValueKind::U64 => radix_transaction::manifest::ast::Type::U64,
            ValueKind::U128 => radix_transaction::manifest::ast::Type::U128,

            ValueKind::String => radix_transaction::manifest::ast::Type::String,

            ValueKind::Enum => radix_transaction::manifest::ast::Type::Enum,
            ValueKind::Option => radix_transaction::manifest::ast::Type::Enum,
            ValueKind::Result => radix_transaction::manifest::ast::Type::Enum,

            ValueKind::Array => radix_transaction::manifest::ast::Type::Array,
            ValueKind::Map => radix_transaction::manifest::ast::Type::Array,
            ValueKind::Tuple => radix_transaction::manifest::ast::Type::Tuple,

            ValueKind::Decimal => radix_transaction::manifest::ast::Type::Decimal,
            ValueKind::PreciseDecimal => radix_transaction::manifest::ast::Type::PreciseDecimal,

            ValueKind::PackageAddress => radix_transaction::manifest::ast::Type::PackageAddress,
            ValueKind::ComponentAddress => radix_transaction::manifest::ast::Type::ComponentAddress,
            ValueKind::ResourceAddress => radix_transaction::manifest::ast::Type::ResourceAddress,
            ValueKind::SystemAddress => radix_transaction::manifest::ast::Type::SystemAddress,

            ValueKind::Hash => radix_transaction::manifest::ast::Type::Hash,

            ValueKind::Bucket => radix_transaction::manifest::ast::Type::Bucket,
            ValueKind::Proof => radix_transaction::manifest::ast::Type::Proof,

            ValueKind::NonFungibleId => radix_transaction::manifest::ast::Type::NonFungibleId,
            ValueKind::NonFungibleAddress => {
                radix_transaction::manifest::ast::Type::NonFungibleAddress
            }

            ValueKind::Blob => radix_transaction::manifest::ast::Type::Blob,
            ValueKind::Bytes => radix_transaction::manifest::ast::Type::Bytes,
            ValueKind::Expression => radix_transaction::manifest::ast::Type::Expression,

            ValueKind::EcdsaSecp256k1PublicKey => {
                radix_transaction::manifest::ast::Type::EcdsaSecp256k1PublicKey
            }
            ValueKind::EcdsaSecp256k1Signature => {
                radix_transaction::manifest::ast::Type::EcdsaSecp256k1Signature
            }
            ValueKind::EddsaEd25519PublicKey => {
                radix_transaction::manifest::ast::Type::EddsaEd25519PublicKey
            }
            ValueKind::EddsaEd25519Signature => {
                radix_transaction::manifest::ast::Type::EddsaEd25519Signature
            }
            ValueKind::Own => radix_transaction::manifest::ast::Type::Own,
        }
    }
}

impl From<radix_transaction::manifest::ast::Type> for ValueKind {
    fn from(value: radix_transaction::manifest::ast::Type) -> ValueKind {
        match value {
            radix_transaction::manifest::ast::Type::Bool => Self::Bool,

            radix_transaction::manifest::ast::Type::I8 => Self::I8,
            radix_transaction::manifest::ast::Type::I16 => Self::I16,
            radix_transaction::manifest::ast::Type::I32 => Self::I32,
            radix_transaction::manifest::ast::Type::I64 => Self::I64,
            radix_transaction::manifest::ast::Type::I128 => Self::I128,
            radix_transaction::manifest::ast::Type::U8 => Self::U8,
            radix_transaction::manifest::ast::Type::U16 => Self::U16,
            radix_transaction::manifest::ast::Type::U32 => Self::U32,
            radix_transaction::manifest::ast::Type::U64 => Self::U64,
            radix_transaction::manifest::ast::Type::U128 => Self::U128,

            radix_transaction::manifest::ast::Type::String => Self::String,

            radix_transaction::manifest::ast::Type::Enum => Self::Enum,

            radix_transaction::manifest::ast::Type::Array => Self::Array,
            radix_transaction::manifest::ast::Type::Tuple => Self::Tuple,

            radix_transaction::manifest::ast::Type::Decimal => Self::Decimal,
            radix_transaction::manifest::ast::Type::PreciseDecimal => Self::PreciseDecimal,

            radix_transaction::manifest::ast::Type::PackageAddress => Self::PackageAddress,
            radix_transaction::manifest::ast::Type::ComponentAddress => Self::ComponentAddress,
            radix_transaction::manifest::ast::Type::ResourceAddress => Self::ResourceAddress,
            radix_transaction::manifest::ast::Type::SystemAddress => Self::SystemAddress,

            radix_transaction::manifest::ast::Type::Hash => Self::Hash,
            radix_transaction::manifest::ast::Type::EcdsaSecp256k1PublicKey => {
                Self::EcdsaSecp256k1PublicKey
            }
            radix_transaction::manifest::ast::Type::EcdsaSecp256k1Signature => {
                Self::EcdsaSecp256k1Signature
            }
            radix_transaction::manifest::ast::Type::EddsaEd25519PublicKey => {
                Self::EddsaEd25519PublicKey
            }
            radix_transaction::manifest::ast::Type::EddsaEd25519Signature => {
                Self::EddsaEd25519Signature
            }

            radix_transaction::manifest::ast::Type::Bucket => Self::Bucket,
            radix_transaction::manifest::ast::Type::Proof => Self::Proof,

            radix_transaction::manifest::ast::Type::NonFungibleId => Self::NonFungibleId,
            radix_transaction::manifest::ast::Type::NonFungibleAddress => Self::NonFungibleAddress,

            radix_transaction::manifest::ast::Type::Blob => Self::Blob,
            radix_transaction::manifest::ast::Type::Expression => Self::Expression,
            radix_transaction::manifest::ast::Type::Bytes => Self::Bytes,
            radix_transaction::manifest::ast::Type::Own => Self::Own,
        }
    }
}

impl From<ValueKind> for ScryptoValueKind {
    fn from(value: ValueKind) -> Self {
        match value {
            ValueKind::Bool => ScryptoValueKind::Bool,

            ValueKind::U8 => ScryptoValueKind::U8,
            ValueKind::U16 => ScryptoValueKind::U16,
            ValueKind::U32 => ScryptoValueKind::U32,
            ValueKind::U64 => ScryptoValueKind::U64,
            ValueKind::U128 => ScryptoValueKind::U128,

            ValueKind::I8 => ScryptoValueKind::I8,
            ValueKind::I16 => ScryptoValueKind::I16,
            ValueKind::I32 => ScryptoValueKind::I32,
            ValueKind::I64 => ScryptoValueKind::I64,
            ValueKind::I128 => ScryptoValueKind::I128,

            ValueKind::String => ScryptoValueKind::String,

            ValueKind::Enum => ScryptoValueKind::Enum,
            ValueKind::Option => ScryptoValueKind::Enum,
            ValueKind::Result => ScryptoValueKind::Enum,

            ValueKind::Map => ScryptoValueKind::Map,
            ValueKind::Array => ScryptoValueKind::Array,
            ValueKind::Bytes => ScryptoValueKind::Array,
            ValueKind::Tuple => ScryptoValueKind::Tuple,

            ValueKind::SystemAddress => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::SystemAddress)
            }
            ValueKind::PackageAddress => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::PackageAddress)
            }
            ValueKind::ResourceAddress => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::ResourceAddress)
            }
            ValueKind::ComponentAddress => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::ComponentAddress)
            }

            ValueKind::Proof => ScryptoValueKind::Custom(ScryptoCustomValueKind::Proof),
            ValueKind::Bucket => ScryptoValueKind::Custom(ScryptoCustomValueKind::Bucket),

            ValueKind::Expression => ScryptoValueKind::Custom(ScryptoCustomValueKind::Expression),
            ValueKind::Blob => ScryptoValueKind::Custom(ScryptoCustomValueKind::Blob),
            ValueKind::NonFungibleAddress => ScryptoValueKind::Tuple,

            ValueKind::Hash => ScryptoValueKind::Custom(ScryptoCustomValueKind::Hash),
            ValueKind::EcdsaSecp256k1PublicKey => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::EcdsaSecp256k1PublicKey)
            }
            ValueKind::EcdsaSecp256k1Signature => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::EcdsaSecp256k1Signature)
            }
            ValueKind::EddsaEd25519PublicKey => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::EddsaEd25519PublicKey)
            }
            ValueKind::EddsaEd25519Signature => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::EddsaEd25519Signature)
            }
            ValueKind::Decimal => ScryptoValueKind::Custom(ScryptoCustomValueKind::Decimal),
            ValueKind::PreciseDecimal => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::PreciseDecimal)
            }
            ValueKind::NonFungibleId => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::NonFungibleId)
            }
            ValueKind::Own => ScryptoValueKind::Custom(ScryptoCustomValueKind::Own),
        }
    }
}

impl From<ScryptoValueKind> for ValueKind {
    fn from(value: ScryptoValueKind) -> Self {
        match value {
            ScryptoValueKind::Bool => ValueKind::Bool,

            ScryptoValueKind::U8 => ValueKind::U8,
            ScryptoValueKind::U16 => ValueKind::U16,
            ScryptoValueKind::U32 => ValueKind::U32,
            ScryptoValueKind::U64 => ValueKind::U64,
            ScryptoValueKind::U128 => ValueKind::U128,

            ScryptoValueKind::I8 => ValueKind::I8,
            ScryptoValueKind::I16 => ValueKind::I16,
            ScryptoValueKind::I32 => ValueKind::I32,
            ScryptoValueKind::I64 => ValueKind::I64,
            ScryptoValueKind::I128 => ValueKind::I128,

            ScryptoValueKind::String => ValueKind::String,

            ScryptoValueKind::Enum => ValueKind::Enum,
            ScryptoValueKind::Map => ValueKind::Map,
            ScryptoValueKind::Array => ValueKind::Array,
            ScryptoValueKind::Tuple => ValueKind::Tuple,

            ScryptoValueKind::Custom(custom_type_id) => match custom_type_id {
                ScryptoCustomValueKind::PackageAddress => ValueKind::PackageAddress,
                ScryptoCustomValueKind::ComponentAddress => ValueKind::ComponentAddress,
                ScryptoCustomValueKind::ResourceAddress => ValueKind::ResourceAddress,
                ScryptoCustomValueKind::SystemAddress => ValueKind::SystemAddress,

                ScryptoCustomValueKind::Bucket => ValueKind::Bucket,
                ScryptoCustomValueKind::Proof => ValueKind::Proof,

                ScryptoCustomValueKind::Expression => ValueKind::Expression,
                ScryptoCustomValueKind::Blob => ValueKind::Blob,

                ScryptoCustomValueKind::Hash => ValueKind::Hash,
                ScryptoCustomValueKind::EcdsaSecp256k1PublicKey => {
                    ValueKind::EcdsaSecp256k1PublicKey
                }
                ScryptoCustomValueKind::EcdsaSecp256k1Signature => {
                    ValueKind::EcdsaSecp256k1Signature
                }
                ScryptoCustomValueKind::EddsaEd25519PublicKey => ValueKind::EddsaEd25519PublicKey,
                ScryptoCustomValueKind::EddsaEd25519Signature => ValueKind::EddsaEd25519Signature,
                ScryptoCustomValueKind::Decimal => ValueKind::Decimal,
                ScryptoCustomValueKind::PreciseDecimal => ValueKind::PreciseDecimal,
                ScryptoCustomValueKind::NonFungibleId => ValueKind::NonFungibleId,
                ScryptoCustomValueKind::Own => ValueKind::Own,
            },
        }
    }
}

// =============================
// From and TryFrom Conversions
// =============================

macro_rules! impl_from_and_try_from_value {
    ($variant_name: ident, $underlying_type: ident, $field: ident) => {
        impl From<$underlying_type> for Value {
            fn from($field: $underlying_type) -> Self {
                Value::$variant_name { $field }
            }
        }

        impl TryFrom<Value> for $underlying_type {
            type Error = Error;

            fn try_from(val: Value) -> Result<Self, Self::Error> {
                match val {
                    Value::$variant_name { $field } => Ok($field),
                    _ => Err(Error::InvalidType {
                        expected_types: vec![ValueKind::$variant_name],
                        actual_type: val.kind(),
                    }),
                }
            }
        }
    };
}

impl_from_and_try_from_value! {Blob, ManifestBlobRef, hash}
impl_from_and_try_from_value! {String, String, value}
impl_from_and_try_from_value! {Decimal, Decimal, value}
impl_from_and_try_from_value! {Proof, ProofId, identifier}
impl_from_and_try_from_value! {Bucket, BucketId, identifier}
impl_from_and_try_from_value! {NonFungibleId, NonFungibleId, value}
impl_from_and_try_from_value! {NonFungibleAddress, NonFungibleAddress, address}
impl_from_and_try_from_value! {PackageAddress, NetworkAwarePackageAddress, address}
impl_from_and_try_from_value! {ResourceAddress, NetworkAwareResourceAddress, address}
impl_from_and_try_from_value! {ComponentAddress, NetworkAwareComponentAddress, address}
impl_from_and_try_from_value! {EcdsaSecp256k1PublicKey, EcdsaSecp256k1PublicKey, public_key}
