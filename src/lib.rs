//! A Rust implementation of the `TypeId`suffix as specified in the `TypeId`Specification.
//!
//! This crate provides functionality to work with `TypeId`suffixes, which are
//! base32-encoded representations of UUIDs used in the `TypeId`system. It supports
//! different UUID versions, with a focus on `UUIDv7`, and provides a flexible
//! architecture for handling various UUID versions.
//!
//! # Features
//!
//! - Create `TypeId`suffixes from UUIDs
//! - Parse `TypeId`suffixes from strings
//! - Convert `TypeId`suffixes back to UUIDs
//! - Support for `UUIDv7` and other UUID versions
//! - Customizable UUID version validation
//!
//! # Usage
//!
//! To use this crate, add it to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! typeid_suffix = "1.0.0"
//! ```
//!
//! Then, in your Rust code:
//!
//! ```
//! use std::str::FromStr;
//! use typeid_suffix::prelude::*;
//! use uuid::Uuid;
//!
//! // Create a `TypeId`suffix from a `UUIDv7`
//! let uuid = Uuid::now_v7();
//! let suffix = TypeIdSuffix::<UuidV7>::new(uuid).expect("Valid `UUIDv7`");
//!
//! // Convert the suffix to a string
//! let suffix_str = suffix.to_string();
//!
//! // Parse a `TypeId`suffix from a string
//! let parsed_suffix = TypeIdSuffix::<UuidV7>::from_str(&suffix_str).expect("Valid suffix");
//!
//! // Convert back to a UUID
//! let recovered_uuid: Uuid = suffix.try_into().expect("Valid UUID");
//! assert_eq!(uuid, recovered_uuid);
//! ```
//!
//! # Modules
//!
//! - `errors`: Defines the error types used throughout the crate.
//! - `encoding`: Provides functions for base32 encoding and decoding.
//! - `traits`: Defines the `UuidVersion` trait for version-specific behavior.
//! - `uuidv7`: Implements `UuidVersion` for `UUIDv7`.
//! - `uuid_other`: Implements `UuidVersion` for other UUID versions.
//! - `typeid_suffix`: Defines the main `TypeIdSuffix` struct and its implementations.
//!
//! # Prelude
//!
//! For convenience, commonly used types and traits are re-exported in the `prelude` module.

mod errors;
mod encoding;
mod traits;
mod uuidv7;
mod uuid_other;
mod typeid_suffix;

/// Convenient re-exports of commonly used types and traits.
///
/// This module re-exports the main types and traits from the crate,
/// allowing users to easily import everything they need with a single
/// use statement.
///
/// # Example
///
/// ```
/// use typeid_suffix::prelude::*;
///
/// // Now you can use TypeIdSuffix, UuidV7, UuidOther, etc. directly
/// ```
pub mod prelude {
    pub use crate::errors::*;
    pub use crate::traits::*;
    pub use crate::typeid_suffix::TypeIdSuffix;
    pub use crate::uuid_other::UuidOther;
    pub use crate::uuidv7::UuidV7;
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use proptest::prelude::*;
    use uuid::{Uuid, Version};

    use crate::prelude::{TypeIdSuffix, UuidOther, UuidV7};

    #[test]
    fn test_typeid_suffix_default() {
        let suffix = TypeIdSuffix::default();
        let uuid: Uuid = suffix.try_into().unwrap();
        assert_eq!(uuid.get_version(), Some(uuid::Version::SortRand));
    }


    #[test]
    fn test_typeid_suffix_explicit_version() {
        let _suffix = TypeIdSuffix::<UuidOther>::new(Uuid::new_v4()).unwrap();
    }

    prop_compose! {
    fn arbitrary_uuidv7()(timestamp in 0..=0xFFFF_FFFF_FFFF_FFFFu64, counter in 0..0x3FFFu16) -> Uuid {
        let mut bytes = [0u8; 16];
        bytes[0..8].copy_from_slice(&timestamp.to_be_bytes());
        bytes[8..10].copy_from_slice(&counter.to_be_bytes());
        bytes[6] = (bytes[6] & 0x0F) | 0x70; // Set version to 7
        bytes[8] = (bytes[8] & 0x3F) | 0x80; // Set variant to RFC4122
        Uuid::from_bytes(bytes)
    }
}

    prop_compose! {
        fn arbitrary_uuid_other()(version in 1u8..=5u8, bytes in proptest::array::uniform16(any::<u8>())) -> Uuid {
            let mut uuid_bytes = bytes;
            uuid_bytes[6] = (uuid_bytes[6] & 0x0F) | (version << 4); // Set version
            uuid_bytes[8] = (uuid_bytes[8] & 0x3F) | 0x80; // Set variant to RFC4122
            Uuid::from_bytes(uuid_bytes)
        }
    }

    proptest! {
        #[test]
        fn test_uuidv7_roundtrip(uuid in arbitrary_uuidv7()) {
            let suffix = TypeIdSuffix::<UuidV7>::new(uuid).unwrap();
            let decoded: Uuid = suffix.clone().try_into().unwrap();
            prop_assert_eq!(uuid, decoded);
            prop_assert_eq!(suffix.len(), 26);
        }

        #[test]
        fn test_uuid_other_roundtrip(uuid in arbitrary_uuid_other()) {
            let suffix = TypeIdSuffix::<UuidOther>::new(uuid).unwrap();
            let decoded: Uuid = suffix.clone().try_into().unwrap();
            prop_assert_eq!(uuid, decoded);
            prop_assert_eq!(suffix.len(), 26);
        }

        #[test]
        fn test_uuidv7_fromstr(uuid in arbitrary_uuidv7()) {
            let suffix = TypeIdSuffix::<UuidV7>::new(uuid).unwrap();
            let from_str = TypeIdSuffix::<UuidV7>::from_str(&suffix).unwrap();
            prop_assert_eq!(suffix, from_str);
        }

        #[test]
        fn test_uuid_other_fromstr(uuid in arbitrary_uuid_other()) {
            let suffix = TypeIdSuffix::<UuidOther>::new(uuid).unwrap();
            let from_str = TypeIdSuffix::<UuidOther>::from_str(&suffix).unwrap();
            prop_assert_eq!(suffix, from_str);
        }

        #[test]
        fn test_invalid_suffix(s in "[0-9a-zA-Z]{26}") {
            if s.as_bytes()[0] > b'7' {
                prop_assert!(TypeIdSuffix::<UuidV7>::from_str(&s).is_err());
                prop_assert!(TypeIdSuffix::<UuidOther>::from_str(&s).is_err());
            }
        }

        #[test]
        fn test_uuidv7_validation(uuid in arbitrary_uuid_other()) {
            if uuid.get_version() != Some(Version::SortRand) {
                prop_assert!(TypeIdSuffix::<UuidV7>::new(uuid).is_err());
            }
        }

        #[test]
        fn test_uuid_other_validation(uuid in arbitrary_uuidv7()) {
            prop_assert!(TypeIdSuffix::<UuidOther>::new(uuid).is_ok());
        }


    }
}