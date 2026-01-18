# ⚠️ DEPRECATED - This repository has been archived

> **This crate has been consolidated into the [mti workspace](https://github.com/GovCraft/mti).**
>
> All future development will occur in the unified workspace repository.

## Migration

The `typeid_suffix` crate is now maintained as part of the `mti` (Magic Type Id) workspace at:

**https://github.com/GovCraft/mti**

The crate continues to be published to crates.io from the new location. Existing dependencies on `typeid_suffix` will continue to work - no changes are required for consumers.

## Why the change?

All three TypeID-related crates (`mti`, `typeid_prefix`, and `typeid_suffix`) are now consolidated into a single Cargo workspace for:

- Unified development and testing
- Consistent versioning and releases
- Simplified maintenance
- Shared CI/CD infrastructure

## Links

- **New Repository**: https://github.com/GovCraft/mti
- **crates.io**: https://crates.io/crates/typeid_suffix (still available)
- **Documentation**: https://docs.rs/typeid_suffix

---

*Original README content preserved below for reference:*

---

# `TypeId` Suffix

[![Crates.io](https://img.shields.io/crates/v/typeid_suffix.svg)](https://crates.io/crates/typeid_suffix)
[![Documentation](https://docs.rs/typeid_suffix/badge.svg)](https://docs.rs/typeid_suffix)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

A Rust library that implements the suffix portion of the [TypeID Specification](https://github.com/jetpack-io/typeid). It provides functionality to work with `TypeId` suffixes, which are base32-encoded representations of UUIDs used in the `TypeId` system.

For a complete TypeID implementation, use the [mti (Magic Type Id) crate](https://crates.io/crates/mti).
