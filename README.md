<h1 align="center">
  <a href="https://www.svix.com">
    <img width="120" src="https://avatars.githubusercontent.com/u/80175132?s=200&v=4" />
    <p align="center">Svix - Webhooks as a service</p>
  </a>
</h1>

# Svix-KSUID (Rust)

A pure Rust and fully tested KSUID implementation

![GitHub tag](https://img.shields.io/github/tag/svix/rust-ksuid.svg)
[![Crates.io](https://img.shields.io/crates/v/svix-ksuid)](https://crates.io/crates/svix-ksuid)
[![Build Status](https://github.com/svix/rust-ksuid/workflows/CI/badge.svg)](https://github.com/svix/rust-ksuid/actions)
[![Security audit](https://github.com/svix/rust-ksuid/actions/workflows/security.yml/badge.svg)](https://github.com/svix/rust-ksuid/actions/workflows/security.yml)
[![docs.rs](https://docs.rs/svix-ksuid/badge.svg)](https://docs.rs/svix-ksuid/)
[![License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE)
[![Join our slack](https://img.shields.io/badge/Slack-join%20the%20community-blue?logo=slack&style=social)](https://www.svix.com/slack/)

This library is fully compatible with [Segment's KSUID](https://segment.com/blog/a-brief-history-of-the-uuid/) implementation:
https://github.com/segmentio/ksuid

For the Python version, please check out https://github.com/svix/python-ksuid

## What is a ksuid?

A ksuid is a K sorted UID. In other words, a KSUID also stores a date component, so that ksuids can be approximately 
sorted based on the time they were created. 

Read more [here](https://segment.com/blog/a-brief-history-of-the-uuid/).

## Usage

Add the dependency:

```toml
[dependencies]
svix-ksuid = "^0.5.0"
```

```rust
use svix_ksuid::*;

let ksuid = Ksuid::new(None, None);
println!("{}", ksuid.to_string());
// 1srOrx2ZWZBpBUvZwXKQmoEYga2
```

### Higher timestamp accuracy mode

Ksuids have a 1 second accuracy which is not sufficient for all use-cases. That's why this library exposes a higher accuracy mode which supports accuracy of up to 4ms.

It's fully compatible with normal ksuids, in fact, it outputs valid ksuids. The difference is that it sacrifices one byte of the random payload in favor of this accuracy.

The code too is fully compatible:

```rust
use svix_ksuid::*;

let ksuid = KsuidMs::new(None, None);
```

And they both implement the same `KsuidLike` trait.

## Examples

### Converting Ksuids

```rust
use svix_ksuid::*;

let ksuid = Ksuid::new(None, None);

// Base62
println!("{}", ksuid.to_string()); // also: ksuid.to_base62()
// 1srOrx2ZWZBpBUvZwXKQmoEYga2

// Bytes (&[u8])
println!("{:?}", ksuid.bytes());
// [13, 53, 196, 51, 225, 147, 62, 55, 242, 117, 112, 135, 99, 173, 199, 116, 90, 245, 231, 242]

// Timestamp (time::OffsetDateTime)
println!("{:?}", ksuid.timestamp());
// 2021-05-21T20:04:03Z

// Timestamp (seconds)
println!("{}", ksuid.timestamp_seconds());
1621627443

// Payload (&[u8])
println!("{:?}", ksuid.bytes());
// [225, 147, 62, 55, 242, 117, 112, 135, 99, 173, 199, 116, 90, 245, 231, 242]
```

### Create Ksuids

```rust
use svix_ksuid::*;

// Timestamp is now, payload is randomly generated
let ksuid = Ksuid::new(None, None);

// Explicitly set either
let bytes = [12u8; Ksuid::PAYLOAD_BYTES];
let ksuid = Ksuid::new(Some(Utc::now()), Some(&bytes));
let ksuid = Ksuid::new(None, Some(&bytes));
let ksuid = Ksuid::new(Some(Utc::now()), None);

// From base62
let base62 = "1srOrx2ZWZBpBUvZwXKQmoEYga2";
let ksuid = Ksuid::from_base62(base62).unwrap();
let ksuid = Ksuid::from_str(base62).unwrap(); // Also implement FromStr

// From bytes
let bytes = [12u8; 20];
let ksuid = Ksuid::from_bytes(bytes.clone());
assert_eq!(&bytes, ksuid.bytes());
```

### Compare and order Ksuids

```rust
use svix_ksuid::*;

let ksuid1 = Ksuid::from_seconds(Some(1_555_555_555), None);
let ksuid2 = Ksuid::from_seconds(Some(1_777_777_777), None);

assert!(ksuid1 < ksuid2);
assert!(ksuid1 <= ksuid2);
assert!(ksuid1 == ksuid1);
assert!(ksuid2 > ksuid1);
assert!(ksuid2 >= ksuid1);
```

### License

ksuid source code is available under an MIT [License](./LICENSE).

All rights reserved to the [Svix webhooks service](https://www.svix.com).
