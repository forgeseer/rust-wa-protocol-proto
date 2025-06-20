# Rust WA Protocol - Proto

[![Crates.io](https://img.shields.io/crates/v/rust-wa-protocol-proto.svg?style=flat-square)](https://crates.io/crates/rust-wa-protocol-proto)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)
[![Repository](https://img.shields.io/badge/github-forgeseer/rust--wa--protocol--proto-blue.svg?style=flat-square)](https://github.com/forgeseer/rust-wa-protocol-proto)

Generated, low-level Rust Protobuf structs for the unofficial WhatsApp Web protocol.

## Overview

This crate provides the raw, pre-compiled Rust data structures generated directly from the WhatsApp Web `.proto` schema files. It uses `prost` and `prost-build` to handle the compilation in a `build.rs` script.

The primary purpose of this crate is to serve as a foundational, versioned dependency for higher-level client libraries that need to interact with the WhatsApp Web API. By separating the protocol definitions into this crate, client applications can enjoy significantly faster compilation times, as the large protobuf schema only needs to be compiled when the protocol definitions themselves change.

## ⚠️ Disclaimer

This project is an unofficial, community-driven effort and is **not affiliated with, endorsed, or sponsored by WhatsApp or its parent company, Meta.**

The use of this library to interact with WhatsApp's servers is a violation of their Terms of Service. **Using any client library that depends on this crate is at your own risk and can lead to a temporary or permanent ban of your WhatsApp account.**

This software is intended for educational and research purposes only. The developers and contributors assume no liability for any consequences arising from its use.

## Usage

This crate is not intended for direct use, but rather as a dependency for a client library. You can add it to your project's `Cargo.toml` to get access to all the necessary WhatsApp data types.

Since this crate is hosted on GitHub (and not yet on Crates.io), you can add it as a Git dependency:

```toml
[dependencies]
rust-wa-protocol-proto = { git = "https://github.com/forgeseer/rust-wa-protocol-proto.git", tag = "v0.1.0" }
```

Be sure to replace `tag = "v0.1.0"` with the specific git tag or commit hash you wish to use for a stable build.

### Example

Once added, you can import and use the generated structs from the `waproto` module:

```rust
use rust_wa_protocol_proto::waproto::AnyStruct;

fn main() {
    let message = AnyStruct {
        // ... populate message fields
        ..Default::default()
    };

    // Now you can work with the message struct
    println!("Created a new Message struct.");
}

```

## How It Works

This crate contains the `.proto` definition files for the WhatsApp Web protocol. A `build.rs` script uses the `prost-build` crate to compile these definitions into native Rust structs at build time. This ensures that any project depending on this crate gets direct, efficient access to all required data models without needing to run the `protoc` compiler itself.
