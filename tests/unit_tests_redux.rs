use std::str::FromStr;

use uuid::Uuid;

use typeid_suffix::prelude::*;

#[test]
fn test_encode_decode_roundtrip_v7() {
    let uuid = TypeIdSuffix::default();
    let decoded = uuid.clone().try_into().unwrap();
    assert_eq!(uuid, decoded);
}

#[test]
fn test_encode_decode_roundtrip_other() {
    let suffix = TypeIdSuffix::new::<V5>();
    let decoded = suffix.clone().try_into().unwrap();
    assert_eq!(suffix, decoded);
}

#[test]
fn test_other_uuid_versions() {
    let uuid_v1 = Uuid::now_v1(&Default::default());
    let uuid_v4 = Uuid::new_v4();
    let uuid_v5 = Uuid::new_v5(&Uuid::NAMESPACE_DNS, Default::default());

    let v1_typeid_suffix: TypeIdSuffix = uuid_v1.into();
    let v4_typeid_suffix: TypeIdSuffix = uuid_v4.into();
    let v5_typeid_suffix: TypeIdSuffix = uuid_v5.into();

    assert_eq!(uuid_v1.as_bytes(), v1_typeid_suffix.to_uuid().as_bytes());
    assert_eq!(uuid_v4.as_bytes(), v4_typeid_suffix.to_uuid().as_bytes());
    assert_eq!(uuid_v5.as_bytes(), v5_typeid_suffix.to_uuid().as_bytes());
}

#[test]
fn test_invalid_first_character() {
    let invalid_suffix = "80000000000000000000000000";
    assert!(TypeIdSuffix::from_str(invalid_suffix).is_err());
}
