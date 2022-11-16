use std::convert::TryFrom;
use std::fmt::Display;
use std::str::FromStr;

use scrypto::misc::copy_u8_array;

use serde::de::Error as DeserializationError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use scrypto::prelude::Hash;

use crate::address::Bech32Manager;
use crate::error::Error;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId(pub (Hash, u32));

impl From<(Hash, u32)> for NodeId {
    fn from(value: (Hash, u32)) -> Self {
        Self(value)
    }
}

impl From<NodeId> for (Hash, u32) {
    fn from(entity_id: NodeId) -> Self {
        entity_id.0
    }
}

impl Serialize for NodeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for NodeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let node_id_string: &str = Deserialize::deserialize(deserializer)?;
        node_id_string
            .parse()
            .map_err(|_| DeserializationError::custom("Failed to parse node id from string"))
    }
}

impl ToString for NodeId {
    fn to_string(&self) -> String {
        let mut node_id_bytes: Vec<u8> = self.0 .0.to_vec();
        node_id_bytes.extend(self.0 .1.to_le_bytes());

        hex::encode(node_id_bytes)
    }
}

impl FromStr for NodeId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let node_id_bytes: Vec<u8> = hex::decode(s)
            .map_err(|_| Error::DeserializationError(format!("Failed to decode node id: {}", s)))?;

        let hash_bytes: &[u8] = &node_id_bytes[0..32];
        let index_bytes: &[u8] = &node_id_bytes[32..];

        let hash: Hash = Hash(copy_u8_array(hash_bytes));
        let index: u32 = u32::from_le_bytes(copy_u8_array(index_bytes));

        Ok(Self((hash, index)))
    }
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
#[serde(untagged)]
pub enum Identifier {
    String(String),
    U32(u32),
}

// Defines a network aware address. This is needed for the serialization and deserialization using
// serde.
macro_rules! define_network_aware_address {
    (
        $underlying_type: ty => $network_aware_struct_ident: ident,
        $encoding_method_ident: ident,
        $decoding_method_ident: ident
    ) => {
        #[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
        pub struct $network_aware_struct_ident {
            pub network_id: u8,
            pub address: $underlying_type,
        }

        impl $network_aware_struct_ident {
            pub fn from_u8_array(data: &[u8], network_id: u8) -> Result<Self, Error> {
                if let Ok(address) = <$underlying_type>::try_from(data) {
                    Ok($network_aware_struct_ident {
                        network_id,
                        address,
                    })
                } else {
                    Err(Error::UnrecognizedAddressFormat)
                }
            }
        }

        impl<'de> Deserialize<'de> for $network_aware_struct_ident {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let address_string: &str = Deserialize::deserialize(deserializer)?;

                let address: Self = address_string
                    .parse()
                    .map_err(|err| DeserializationError::custom(format!("{:?}", err)))?;
                Ok(address)
            }
        }

        impl Serialize for $network_aware_struct_ident {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }

        impl From<$network_aware_struct_ident> for $underlying_type {
            fn from(address: $network_aware_struct_ident) -> $underlying_type {
                address.address
            }
        }

        impl From<&$network_aware_struct_ident> for $underlying_type {
            fn from(address: &$network_aware_struct_ident) -> $underlying_type {
                address.address
            }
        }

        impl Display for $network_aware_struct_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let bech32_manager: Bech32Manager = Bech32Manager::new(self.network_id);
                write!(
                    f,
                    "{}",
                    bech32_manager.encoder.$encoding_method_ident(&self.address)
                )
            }
        }

        impl FromStr for $network_aware_struct_ident {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let bech32_manager: Bech32Manager = Bech32Manager::new_from_address(s)?;
                Ok(Self {
                    address: bech32_manager.decoder.$decoding_method_ident(s)?,
                    network_id: bech32_manager.network_id(),
                })
            }
        }
    };
}

define_network_aware_address!(
    scrypto::prelude::ComponentAddress => NetworkAwareComponentAddress,
    encode_component_address_to_string,
    validate_and_decode_component_address
);
define_network_aware_address!(
    scrypto::prelude::PackageAddress => NetworkAwarePackageAddress,
    encode_package_address_to_string,
    validate_and_decode_package_address
);
define_network_aware_address!(
    scrypto::prelude::ResourceAddress => NetworkAwareResourceAddress,
    encode_resource_address_to_string,
    validate_and_decode_resource_address
);
define_network_aware_address!(
    scrypto::prelude::SystemAddress => NetworkAwareSystemAddress,
    encode_system_address_to_string,
    validate_and_decode_system_address
);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "address")]
pub enum Address {
    ComponentAddress(NetworkAwareComponentAddress),
    ResourceAddress(NetworkAwareResourceAddress),
    PackageAddress(NetworkAwarePackageAddress),
}

impl Address {
    pub fn kind(&self) -> AddressKind {
        match self {
            Self::ComponentAddress(component_address) => match component_address.address {
                scrypto::prelude::ComponentAddress::Normal(_) => AddressKind::NormalComponent,
                scrypto::prelude::ComponentAddress::Account(_) => AddressKind::AccountComponent,
                scrypto::prelude::ComponentAddress::EcdsaSecp256k1VirtualAccount(_) => {
                    AddressKind::EcdsaSecp256k1VirtualAccount
                }
                scrypto::prelude::ComponentAddress::EddsaEd25519VirtualAccount(_) => {
                    AddressKind::EddsaEd25519VirtualAccount
                }
            },
            Self::ResourceAddress(resource_address) => match resource_address.address {
                scrypto::prelude::ResourceAddress::Normal(_) => AddressKind::Resource,
            },
            Self::PackageAddress(package_address) => match package_address.address {
                scrypto::prelude::PackageAddress::Normal(_) => AddressKind::Package,
            },
        }
    }

    pub fn network_id(&self) -> u8 {
        match self {
            Self::ComponentAddress(address) => address.network_id,
            Self::ResourceAddress(address) => address.network_id,
            Self::PackageAddress(address) => address.network_id,
        }
    }

    pub fn from_u8_array(array: &[u8], network_id: u8) -> Result<Self, Error> {
        if let Ok(component_address) =
            NetworkAwareComponentAddress::from_u8_array(array, network_id)
        {
            Ok(Self::ComponentAddress(component_address))
        } else if let Ok(resource_address) =
            NetworkAwareResourceAddress::from_u8_array(array, network_id)
        {
            Ok(Self::ResourceAddress(resource_address))
        } else if let Ok(package_address) =
            NetworkAwarePackageAddress::from_u8_array(array, network_id)
        {
            Ok(Self::PackageAddress(package_address))
        } else {
            Err(Error::UnrecognizedAddressFormat)
        }
    }
}

impl From<NetworkAwareComponentAddress> for Address {
    fn from(address: NetworkAwareComponentAddress) -> Self {
        Self::ComponentAddress(address)
    }
}

impl From<NetworkAwareResourceAddress> for Address {
    fn from(address: NetworkAwareResourceAddress) -> Self {
        Self::ResourceAddress(address)
    }
}

impl From<NetworkAwarePackageAddress> for Address {
    fn from(address: NetworkAwarePackageAddress) -> Self {
        Self::PackageAddress(address)
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Address::ComponentAddress(address) => write!(f, "{}", address),
            Address::ResourceAddress(address) => write!(f, "{}", address),
            Address::PackageAddress(address) => write!(f, "{}", address),
        }
    }
}

impl FromStr for Address {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(component_address) = NetworkAwareComponentAddress::from_str(s) {
            Ok(Self::ComponentAddress(component_address))
        } else if let Ok(resource_address) = NetworkAwareResourceAddress::from_str(s) {
            Ok(Self::ResourceAddress(resource_address))
        } else if let Ok(package_address) = NetworkAwarePackageAddress::from_str(s) {
            Ok(Self::PackageAddress(package_address))
        } else {
            Err(Error::UnrecognizedAddressFormat)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AddressKind {
    Resource,
    Package,

    NormalComponent,
    AccountComponent,
    EcdsaSecp256k1VirtualAccount,
    EddsaEd25519VirtualAccount,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "variant")]
pub enum OptionProxy<T> {
    Some { field: T },
    None,
}

impl<T> From<Option<T>> for OptionProxy<T> {
    fn from(option: Option<T>) -> Self {
        match option {
            Option::Some(field) => Self::Some { field },
            Option::None => Self::None,
        }
    }
}

impl<T> From<OptionProxy<T>> for Option<T> {
    fn from(option: OptionProxy<T>) -> Self {
        match option {
            OptionProxy::Some { field } => Self::Some(field),
            OptionProxy::None => Self::None,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "variant")]
pub enum ResultProxy<O, E> {
    Ok { field: O },
    Err { field: E },
}

impl<O, E> From<ResultProxy<O, E>> for Result<O, E> {
    fn from(result: ResultProxy<O, E>) -> Self {
        match result {
            ResultProxy::Ok { field } => Result::Ok(field),
            ResultProxy::Err { field } => Result::Err(field),
        }
    }
}

impl<O, E> From<Result<O, E>> for ResultProxy<O, E> {
    fn from(result: Result<O, E>) -> Self {
        match result {
            Result::Ok(field) => ResultProxy::Ok { field },
            Result::Err(field) => ResultProxy::Err { field },
        }
    }
}