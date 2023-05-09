// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use radix_engine_toolkit::model::value::ast::{ManifestAstValue, ManifestAstValueKind};
use radix_engine_toolkit::visitor::{traverse_value, ValueAliasingVisitor};
use scrypto::prelude::IntegerNonFungibleLocalId;

#[test]
fn aliasing_of_deeply_nested_structures_works() {
    // Arrange
    let mut value = ManifestAstValue::Map {
        key_value_kind: ManifestAstValueKind::String,
        value_value_kind: ManifestAstValueKind::Tuple,
        entries: vec![
            (
                ManifestAstValue::String {
                    value: "HelloWorld".into(),
                },
                ManifestAstValue::Tuple {
                    elements: vec![
                        ManifestAstValue::Decimal {
                            value: "12".parse().unwrap(),
                        },
                        ManifestAstValue::PreciseDecimal {
                            value: "12".parse().unwrap(),
                        },
                    ],
                },
            ),
            (
                ManifestAstValue::String {
                    value: "WorldHello".into(),
                },
                ManifestAstValue::Tuple {
                    elements: vec![ManifestAstValue::Tuple {
                        elements: vec![ManifestAstValue::Tuple {
                            elements: vec![ManifestAstValue::Array {
                                element_kind: ManifestAstValueKind::Array,
                                elements: vec![ManifestAstValue::Array {
                                    element_kind: ManifestAstValueKind::Tuple,
                                    elements: vec![ManifestAstValue::Tuple { elements: vec![
                                        ManifestAstValue::Address { address: "resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3".parse().unwrap() },
                                        ManifestAstValue::NonFungibleLocalId { value: scrypto::prelude::NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1)) } ,
                                    ] }],
                                }],
                            }],
                        }],
                    }],
                },
            ),
        ],
    };

    let expected = ManifestAstValue::Map {
        key_value_kind: ManifestAstValueKind::String,
        value_value_kind: ManifestAstValueKind::Tuple,
        entries: vec![
            (
                ManifestAstValue::String {
                    value: "HelloWorld".into(),
                },
                ManifestAstValue::Tuple {
                    elements: vec![
                        ManifestAstValue::Decimal {
                            value: "12".parse().unwrap(),
                        },
                        ManifestAstValue::PreciseDecimal {
                            value: "12".parse().unwrap(),
                        },
                    ],
                },
            ),
            (
                ManifestAstValue::String {
                    value: "WorldHello".into(),
                },
                ManifestAstValue::Tuple {
                    elements: vec![ManifestAstValue::Tuple {
                        elements: vec![ManifestAstValue::Tuple {
                            elements: vec![ManifestAstValue::Array {
                                element_kind: ManifestAstValueKind::Array,
                                elements: vec![ManifestAstValue::Array {
                                    element_kind: ManifestAstValueKind::Tuple,
                                    elements: vec![ManifestAstValue::NonFungibleGlobalId {
                                        resource_address: "resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3".parse().unwrap(),
                                        non_fungible_local_id: scrypto::prelude::NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1))
                                    }],
                                }],
                            }],
                        }],
                    }],
                },
            ),
        ],
    };

    let mut visitor = ValueAliasingVisitor::default();

    // Act
    traverse_value(&mut value, &mut [&mut visitor]).unwrap();

    // Assert
    assert_eq!(expected, value)
}
