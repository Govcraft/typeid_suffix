use uuid::{Uuid, Version};

/// Trait for UUID version-specific behavior in `TypeId`suffix operations.
///
/// This trait defines the interface for handling different UUID versions
/// within the `TypeId`suffix context. Implementations of this trait provide
/// version-specific validation and identification.
///
/// Implementors of this trait can be used as type parameters for `TypeIdSuffix`,
/// allowing for flexible handling of different UUID versions.
///
/// # Examples
///
/// Implementing `UuidVersion` for a custom UUID version:
///
/// ```
/// use uuid::{Uuid, Version};
/// use typeid_suffix::prelude::*;
///
/// struct MyCustomUuidVersion;
///
/// impl UuidVersion for MyCustomUuidVersion {
///     const VERSION: Option<Version> = Some(Version::Random);  // Example: Using v4
///
///     fn validate(uuid: &Uuid) -> bool {
///         uuid.get_version() == Some(Version::Random)
///     }
/// }
/// ```
pub trait UuidVersion {
    /// The corresponding UUID version.
    ///
    /// This constant should be set to the specific `Version` that the implementor
    /// represents. If the implementor can handle multiple versions or doesn't
    /// correspond to a specific version, this can be set to `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::Version;
    /// use typeid_suffix::prelude::*;
    ///
    /// struct UuidV4;
    ///
    /// impl UuidVersion for UuidV4 {
    ///     const VERSION: Option<Version> = Some(Version::Random);
    ///     // ... other implementation details
    /// #    fn validate(_: &uuid::Uuid) -> bool { true }
    /// }
    /// ```
    const VERSION: Option<Version>;

    /// Validates a UUID according to version-specific rules.
    ///
    /// This method should implement the logic to determine whether a given UUID
    /// is valid for the specific version (or versions) that the implementor represents.
    ///
    /// # Arguments
    ///
    /// * `uuid`: A reference to the `Uuid` to be validated.
    ///
    /// # Returns
    ///
    /// Returns `true` if the UUID is valid for this version, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::{Uuid, Version};
    /// use typeid_suffix::prelude::*;
    ///
    /// struct UuidV7;
    ///
    /// impl UuidVersion for UuidV7 {
    ///     const VERSION: Option<Version> = Some(Version::SortRand);
    ///
    ///     fn validate(uuid: &Uuid) -> bool {
    ///         uuid.get_version() == Some(Version::SortRand)
    ///     }
    /// }
    ///
    /// let uuid = Uuid::now_v7();
    /// assert!(UuidV7::validate(&uuid));
    /// ```
    fn validate(uuid: &Uuid) -> bool;
}