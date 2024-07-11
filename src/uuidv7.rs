use uuid::{Uuid, Variant, Version};
use crate::traits::UuidVersion;

/// A marker struct representing UUID version 7.
///
/// UUID version 7 is a time-ordered UUID that uses a 48-bit timestamp and up to 74 bits
/// of additional randomness. It's designed to be monotonic and sortable while maintaining
/// the benefits of UUID uniqueness.
///
/// This struct implements the `UuidVersion` trait, providing version-specific validation
/// for `UUIDv7`.
///
/// # Examples
///
/// ```
/// use uuid::Uuid;
/// use typeid_suffix::prelude::*;
///
/// let uuid = Uuid::now_v7();
/// assert!(UuidV7::validate(&uuid));
///
/// let uuid_v4 = Uuid::new_v4();
/// assert!(!UuidV7::validate(&uuid_v4));
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UuidV7;

impl UuidVersion for UuidV7 {
    /// The UUID version associated with this type.
    ///
    /// For `UuidV7`, this is always `Some(Version::SortRand)`.
    const VERSION: Option<Version> = Some(Version::SortRand);

    /// Validates whether a given UUID is a valid version 7 UUID.
    ///
    /// This method checks two criteria:
    /// 1. The UUID version is 7 (`SortRand`).
    /// 2. The UUID variant is RFC4122.
    ///
    /// # Arguments
    ///
    /// * `uuid`: A reference to the UUID to validate.
    ///
    /// # Returns
    ///
    /// Returns `true` if the UUID is a valid version 7 UUID, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::Uuid;
    /// use typeid_suffix::prelude::*;
    ///
    /// let uuid_v7 = Uuid::now_v7();
    /// assert!(UuidV7::validate(&uuid_v7));
    ///
    /// let uuid_v4 = Uuid::new_v4();
    /// assert!(!UuidV7::validate(&uuid_v4));
    /// ```
    fn validate(uuid: &Uuid) -> bool {
        if uuid.get_version() != Some(Version::SortRand) {
            return false; // Could return Error::InvalidUuid(InvalidUuidReason::InvalidVersion)
        }
        if uuid.get_variant() != Variant::RFC4122 {
            return false; // Could return Error::InvalidUuid(InvalidUuidReason::InvalidVariant)
        }
        true
    }
}