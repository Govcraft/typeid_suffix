//! Error types for the `TypeId`suffix operations.
//!
//! This module defines the error types used throughout the crate to represent
//! various failure cases that can occur during `TypeId`suffix manipulations.
use std::fmt;

#[cfg(feature = "instrument")]
use tracing::error;

/// Represents errors that can occur during `TypeId`suffix operations.
///
/// This enum encapsulates two main categories of errors:
/// - Issues with the `TypeId`suffix itself (`InvalidSuffix`)
/// - Problems related to the underlying UUID (`InvalidUuid`)
///
/// Each variant contains a more specific reason for the error, allowing for
/// detailed error reporting and handling.
///
/// # Examples
///
/// ```
/// use typeid_suffix::prelude::*;
///
/// let suffix_error = DecodeError::InvalidSuffix(InvalidSuffixReason::InvalidLength);
/// assert_eq!(
///     suffix_error.to_string(),
///     "Invalid `TypeId`suffix: Suffix must be exactly 26 characters long"
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    /// Represents an error with the `TypeId`suffix.
    InvalidSuffix(InvalidSuffixReason),
    /// Represents an error with the underlying UUID.
    InvalidUuid(InvalidUuidReason),
}

/// Specifies the reason for an invalid `TypeId`suffix.
///
/// This enum provides more granular information about why a `TypeId`suffix
/// is considered invalid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidSuffixReason {
    /// The suffix does not have the required length of 26 characters.
    InvalidLength,
    /// The suffix contains one or more non-ASCII characters.
    NonAsciiCharacter,
    /// The first character of the suffix is greater than '7'.
    InvalidFirstCharacter,
    /// The suffix contains a character that is not in the base32 alphabet.
    InvalidCharacter,
}

/// Specifies the reason for an invalid UUID.
///
/// This enum provides more detailed information about why a UUID
/// is considered invalid in the context of `TypeId`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidUuidReason {
    /// The UUID version is not valid for this `TypeId`.
    InvalidVersion,
    /// The UUID variant is not RFC4122.
    InvalidVariant,
    /// The UUID bytes are invalid.
    InvalidBytes,
}

impl std::fmt::Display for DecodeError {
    /// Formats the error message for display.
    ///
    /// This implementation provides a human-readable error message that includes both the error
    /// category (`InvalidSuffix` or `InvalidUuid`) and the specific error description.
    ///
    /// # Examples
    ///
    /// ```
    /// use typeid_suffix::prelude::*;
    ///
    /// let uuid_error = DecodeError::InvalidUuid(InvalidUuidReason::InvalidVersion);
    /// assert_eq!(
    ///     uuid_error.to_string(),
    ///     "Invalid UUID: UUID version is not valid for this `TypeId`"
    /// );
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::InvalidSuffix(reason) => format!("Invalid `TypeId`suffix: {reason}"),
            Self::InvalidUuid(reason) => format!("Invalid UUID: {reason}"),
        };

        #[cfg(feature = "instrument")]
        error!("{msg}");

        write!(f, "{msg}")
    }
}


impl std::fmt::Display for InvalidSuffixReason {
    /// Provides a human-readable description of the invalid suffix reason.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::InvalidLength => "Suffix must be exactly 26 characters long",
            Self::NonAsciiCharacter => "Suffix contains non-ASCII characters",
            Self::InvalidFirstCharacter => "First character of suffix must be '7' or less",
            Self::InvalidCharacter => "Suffix contains characters not in the base32 alphabet",
        };

        #[cfg(feature = "instrument")]
        error!("{}", msg);

        write!(f, "{msg}")
    }
}

impl std::fmt::Display for InvalidUuidReason {
    /// Provides a human-readable description of the invalid UUID reason.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::InvalidVersion => "UUID version is not valid for this `TypeId`",
            Self::InvalidVariant => "UUID variant is not RFC4122",
            Self::InvalidBytes => "UUID bytes are invalid",
        };

        #[cfg(feature = "instrument")]
        error!("{msg}");

        write!(f, "{msg}")
    }
}

/// Implements the standard error trait for the `Error` enum.
///
/// This implementation allows `Error` to be used with the standard error handling mechanisms in Rust.
/// It enables the use of `?` operator, `From` trait implementations, and integration with other
/// error handling utilities in the standard library and third-party crates.
impl std::error::Error for DecodeError {}