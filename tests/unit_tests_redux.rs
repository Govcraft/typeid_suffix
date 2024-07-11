use std::str::FromStr;

use uuid::{Context, Timestamp, Uuid};

use typeid_suffix::prelude::*;

#[test]
fn test_encode_decode_roundtrip_v7() {
    let uuid = Uuid::now_v7();
    let suffix = TypeIdSuffix::new(uuid).unwrap();
    let decoded: Uuid = suffix.try_into().unwrap();
    assert_eq!(uuid, decoded);
}

#[test]
fn test_encode_decode_roundtrip_other() {
    let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, "some_string".as_ref());
    let suffix = TypeIdSuffix::new(uuid).expect("Failed to create TypeIdSuffix");
    let decoded: Uuid = suffix.try_into().unwrap();
    assert_eq!(uuid, decoded);
}

#[test]
fn test_other_uuid_versions() {
    let uuid_v1 = Uuid::new_v1(Timestamp::now(Context::new_random()), &[0, 0, 0, 0, 0, 0]);
    let uuid_v4 = Uuid::new_v4();
    let uuid_v5 = Uuid::new_v5(&Uuid::NAMESPACE_DNS, "test".as_bytes());

    assert!(TypeIdSuffix::new(uuid_v1).is_ok());
    assert!(TypeIdSuffix::new(uuid_v4).is_ok());
    assert!(TypeIdSuffix::new(uuid_v5).is_ok());
}

#[test]
fn test_invalid_first_character() {
    let invalid_suffix = "80000000000000000000000000";
    assert!(TypeIdSuffix::from_str(invalid_suffix).is_err());
}
