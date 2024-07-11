use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

use uuid::{Uuid, Variant, Version};

use crate::encoding::{decode_base32, encode_base32};
use crate::errors::{DecodeError, InvalidSuffixReason, InvalidUuidReason};

/// Represents the suffix part of a `TypeId`.
///
/// A `TypeIdSuffix` encapsulates the base32-encoded representation of a UUID,
/// which forms the suffix part of a `TypeId`. It is generic over a UUID version,
/// defaulting to `UUIDv7`.
///
/// # Type Parameters
///
/// * `V`: A type implementing the `UuidVersion` trait, defaulting to `UuidV7`.
///
/// # Examples
///
/// Creating a new `TypeIdSuffix`:
///
/// ```
/// use uuid::Uuid;
/// use typeid_suffix::prelude::*;
///
/// let uuid = Uuid::now_v7();
/// let suffix: TypeIdSuffix = TypeIdSuffix::new(uuid).expect("Valid `UUIDv7`");
/// println!("TypeID suffix: {}", suffix);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeIdSuffix([u8; 26]);

impl TypeIdSuffix {
    /// Creates a new `TypeIdSuffix` from a UUID.
    ///
    /// This method encodes the given UUID into a base32 representation and wraps it
    /// in a `TypeIdSuffix`. The UUID must be valid for the specified version `V`.
    ///
    /// # Arguments
    ///
    /// * `uuid`: The UUID to encode into a `TypeId`suffix.
    ///
    /// # Returns
    ///
    /// Returns a `Result` which is:
    /// - `Ok(TypeIdSuffix)` if the UUID is valid for the specified version.
    /// - `Err(Error::InvalidUuid)` if the UUID doesn't meet the version-specific criteria.
    ///
    /// # Errors
    ///
    /// This function will return an error if the UUID does not meet the version-specific criteria.
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::Uuid;
    /// use typeid_suffix::prelude::*;
    ///
    /// let uuid = Uuid::now_v7();
    /// let suffix = TypeIdSuffix::new(uuid).expect("Valid `UUIDv7`");
    /// ```
    #[cfg_attr(feature = "instrument", tracing::instrument)]
    #[inline]
    pub fn new(uuid: Uuid) -> Result<Self, DecodeError> {
        if !Self::is_valid_uuid(&uuid) {
            return Err(DecodeError::InvalidUuid(InvalidUuidReason::InvalidVersion));
        }
        Ok(Self(encode_base32(uuid.as_bytes())))
    }

    fn is_valid_uuid(uuid: &Uuid) -> bool {
        let is_valid_variant = matches!(
        uuid.get_variant(),
        Variant::RFC4122 | Variant::Microsoft | Variant::Future | Variant::NCS
    );

        let is_valid_version = matches!(
        uuid.get_version(),
        Some(
            Version::Max
                | Version::Custom
                | Version::SortMac
                | Version::Mac
                | Version::Dce
                | Version::Md5
                | Version::Random
                | Version::Sha1
                | Version::SortRand
                | Version::Nil
        )
    );

        is_valid_variant || is_valid_version
    }

    /// Converts the `TypeIdSuffix` back into a UUID.
    ///
    /// This method decodes the base32-encoded suffix back into a UUID.
    ///
    /// # Returns
    ///
    /// Returns a `Result` which is:
    /// - `Ok(Uuid)` if the decoding is successful and the resulting UUID is valid for the version.
    /// - `Err(Error)` if decoding fails or the resulting UUID is invalid for the version.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The base32 decoding fails.
    /// - The resulting UUID does not meet the version-specific criteria.
    #[inline]
    fn to_uuid(&self) -> Result<Uuid, DecodeError> {
        let decoded_bytes = decode_base32(&self.0)?;
        let uuid = Uuid::from_bytes(decoded_bytes);

        if !Self::is_valid_uuid(&uuid) {
            return Err(DecodeError::InvalidUuid(InvalidUuidReason::InvalidVersion));
        }
        Ok(uuid)
    }

    /// Returns a string slice of the base32-encoded suffix.
    ///
    /// # Returns
    ///
    /// A string slice containing the base32-encoded representation of the UUID.
    ///
    /// # Panics
    ///
    /// This function will panic if the internal bytes are not valid UTF-8.
    /// This should never happen as the internal bytes are guaranteed to be ASCII.
    #[must_use]
    #[inline]
    fn as_str(&self) -> &str {
        // SAFETY: This unwrap is safe because we know that the internal bytes
        // are always valid ASCII characters, which are valid UTF-8
        std::str::from_utf8(&self.0).unwrap()
    }
}

impl Default for TypeIdSuffix {
    /// Creates a new `TypeIdSuffix` with a fresh `UUIDv7`.
    ///
    /// # Examples
    ///
    /// ```
    /// use typeid_suffix::prelude::*;
    ///
    /// let suffix = TypeIdSuffix::default();
    /// ```
    fn default() -> Self {
        let uuid = Uuid::now_v7();
        Self::new(uuid).expect("Generated `UUIDv7` should always be valid")
    }
}

impl Deref for TypeIdSuffix {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AsRef<str> for TypeIdSuffix {
    fn as_ref(&self) -> &str {
        self
    }
}

impl Borrow<str> for TypeIdSuffix {
    fn borrow(&self) -> &str {
        self
    }
}

impl fmt::Display for TypeIdSuffix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self)
    }
}

impl TryFrom<&TypeIdSuffix> for Uuid {
    type Error = DecodeError;

    fn try_from(value: &TypeIdSuffix) -> Result<Self, Self::Error> {
        value.to_uuid()
    }
}

impl TryFrom<TypeIdSuffix> for Uuid {
    type Error = DecodeError;

    fn try_from(value: TypeIdSuffix) -> Result<Self, Self::Error> {
        value.to_uuid()
    }
}

impl FromStr for TypeIdSuffix {
    type Err = DecodeError;

    /// Parses a string slice into a `TypeIdSuffix`.
    ///
    /// # Arguments
    ///
    /// * `s`: The string slice to parse.
    ///
    /// # Returns
    ///
    /// Returns a `Result` which is:
    /// - `Ok(TypeIdSuffix)` if the string is a valid base32-encoded `TypeId`suffix.
    /// - `Err(Error)` if the string is not a valid `TypeId`suffix.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The string is not exactly 26 characters long.
    /// - The string contains non-ASCII characters.
    /// - The first character is greater than '7'.
    /// - The string contains characters not in the base32 alphabet.
    /// - The decoded UUID is not valid for the specified version.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use typeid_suffix::prelude::*;
    ///
    /// let suffix = TypeIdSuffix::from_str("01h455vb4pex5vsknk084sn02q").expect("Valid `TypeId`suffix");
    /// ```
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.len() != 26 {
            return Err(DecodeError::InvalidSuffix(InvalidSuffixReason::InvalidLength));
        }
        if !input.is_ascii() {
            return Err(DecodeError::InvalidSuffix(InvalidSuffixReason::NonAsciiCharacter));
        }
        if input.as_bytes()[0] > b'7' {
            return Err(DecodeError::InvalidSuffix(InvalidSuffixReason::InvalidFirstCharacter));
        }
        let encoded_bytes: [u8; 26] = input.as_bytes().try_into().map_err(|_| DecodeError::InvalidSuffix(InvalidSuffixReason::InvalidLength))?;
        let decoded_bytes = decode_base32(&encoded_bytes)?;
        let uuid = Uuid::from_bytes(decoded_bytes);
        if !Self::is_valid_uuid(&uuid) {
            return Err(DecodeError::InvalidUuid(InvalidUuidReason::InvalidVersion));
        }
        Ok(Self(encoded_bytes))
    }
}