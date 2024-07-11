use uuid::{Uuid, Version};
use crate::traits::UuidVersion;

/// Marker struct for UUID versions other than v7.
///
/// This struct implements the `UuidVersion` trait to allow `TypeIdSuffix`
/// to work with UUID versions other than v7. It provides a more permissive
/// validation compared to the `UuidV7` implementation.
///
/// # Examples
///
/// ```
/// use uuid::Uuid;
/// use typeid_suffix::prelude::*;
///
/// let uuid = Uuid::new_v4();  // Using a v4 UUID as an example
/// let suffix = TypeIdSuffix::<UuidOther>::new(uuid).expect("Valid UUID");
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UuidOther;

impl UuidVersion for UuidOther {
    /// Indicates that this implementation doesn't correspond to a specific UUID version.
    const VERSION: Option<Version> = None;

    /// Validates a UUID for use with `TypeIdSuffix<UuidOther>`.
    ///
    /// In the current implementation, this method always returns `true`,
    /// effectively allowing any UUID to be used. This is due to limitations
    /// in the current `TypeId`specification (v3) which doesn't provide
    /// strict guidelines for non-v7 UUIDs.
    ///
    /// # Arguments
    ///
    /// * `_uuid`: A reference to the UUID to validate. The underscore prefix
    ///   indicates that this parameter is currently unused.
    ///
    /// # Returns
    ///
    /// Always returns `true`, indicating that any UUID is considered valid.
    ///
    /// # Note
    ///
    /// Future versions of this implementation may introduce stricter validation.
    /// The commented-out code in this method shows a potential implementation
    /// that would check for RFC4122 variant and valid version numbers
    /// (1, 2, 3, 4, 5, or 7).
    fn validate(_uuid: &Uuid) -> bool {
        // This is what I'd like to do but unfortunately
        // V3 of the typeid specification is not strict enough
        // to allow for this. So we'll just return true for now
        // and let the rest of the code handle the decoding validation
        // // Check that the variant is RFC4122
        // if uuid.get_variant() != Variant::RFC4122 {
        //     return false; // Could return Error::InvalidUuid(InvalidUuidReason::InvalidVariant)
        // }
        //
        // // Check that the version is valid (1, 2, 3, 4, 5, or 7)
        // if !matches!(
        //     uuid.get_version(),
        //     Some(Version::SortMac | Version::Mac | Version::Dce | Version::Md5 | Version::Random | Version::Sha1 | Version::SortRand)
        // )
        true
    }
}