# `TypeId`Suffix

[![Crates.io](https://img.shields.io/crates/v/typeid_suffix.svg)](https://crates.io/crates/typeid_suffix)
[![Documentation](https://docs.rs/typeid_suffix/badge.svg)](https://docs.rs/typeid_suffix)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

A Rust library that implements the suffix portion of the [TypeID Specification](https://github.com/jetpack-io/typeid). It provides functionality to work with `TypeId`suffixes, which are base32-encoded representations of UUIDs used in the `TypeId`system.

Combined with the [TypeIdPrefix crate](https://crates.io/crates/typeid_prefix) to comprise the [mti (Magic Type Id) crate](https://crates.io/crates/mti).

Use the [mti (Magic Type Id) crate](https://crates.io/crates/mti) for a holistic implementation of the TypeID specification.

## Features

- **UUID Version Support**: Implements support for `UUIDv7` and other UUID versions.
- **Flexible Architecture**: Generic implementation allows for handling various UUID versions.
- **Base32 Encoding/Decoding**: Efficient encoding and decoding of UUIDs to/from base32 `TypeId`suffixes.
- **Error Handling**: Comprehensive error types for invalid suffixes and UUIDs.
- **Validation**: Robust validation for `TypeId`suffixes and UUIDs.
- **Zero-cost Abstractions**: Designed to have minimal runtime overhead.
- **Optional Tracing**: Integrates with the `tracing` crate for logging (optional feature).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
typeid_suffix = "0.1.0"
```

To enable optional features:

```toml
[dependencies]
typeid_suffix = { version = "0.1.0", features = ["instrument", "arbitrary"] }
```

## Usage

### Basic Usage

```rust
use std::str::FromStr;
use typeid_suffix::prelude::*;
use uuid::Uuid;

fn main() {
    // Create a `TypeId`suffix from a `UUIDv7`
    let uuid = Uuid::now_v7();
    let suffix = TypeIdSuffix::new(uuid).expect("Valid `UUIDv7`");
    println!("TypeID suffix: {}", suffix);

    // Parse a `TypeId`suffix from a string
    let parsed_suffix = TypeIdSuffix::from_str("01h455vb4pex5vsknk084sn02q").expect("Valid suffix");
    
    // Convert back to a UUID
    let recovered_uuid: Uuid = suffix.try_into().expect("Valid UUID");
    assert_eq!(uuid, recovered_uuid);
}
```

### Working with Other UUID Versions

```rust
use typeid_suffix::prelude::*;
use uuid::Uuid;

fn main() {
    let uuid = Uuid::new_v4();
    let suffix = TypeIdSuffix::new(uuid).expect("Valid UUID");
    println!("TypeID suffix for UUIDv4: {}", suffix);
}
```

### Error Handling

The crate provides detailed error types for various failure cases:

```rust
use typeid_suffix::prelude::*;
use std::str::FromStr;

fn main() {
    let result = TypeIdSuffix::from_str("invalid_suffix");
    match result {
        Ok(_) => println!("Valid suffix"),
        Err(e) => println!("Invalid suffix: {}", e),
    }
}
```

### Optional Tracing

When the `instrument` feature is enabled, the crate will log operations using the `tracing` crate:

```toml
[dependencies]
typeid_suffix = { version = "0.1.0", features = ["instrument"] }
```

## Use Cases

- **Distributed Systems**: Generate globally unique, sortable identifiers for distributed systems.
- **Database Systems**: Create compact, base32-encoded identifiers for database records.
- **API Development**: Use `TypeId`suffixes as part of API responses or for resource identification.
- **Time-based Sorting**: Leverage the sortable nature of `UUIDv7`-based `TypeId`suffixes for time-ordered data.

## Safety and Correctness

This crate has been thoroughly tested and verified:

- Comprehensive unit tests
- Property-based testing with `proptest`
- Fuzz testing

These measures ensure that the crate behaves correctly and safely under various inputs and conditions.

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on Rust 1.60.0 and later.

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Credits

This crate implements a portion of the [TypeID Specification](https://github.com/jetpack-io/typeid) created by Jetpack.io.