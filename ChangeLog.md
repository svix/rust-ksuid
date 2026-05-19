# Changelog

## Version 0.10.0
* Remove internal `MinimalTimestamp` type and use [`std::time::SystemTime`](https://doc.rust-lang.org/std/time/struct.SystemTime.html) as the fallback timestamp
* Rename `TimeStamp` trait to `Timestamp`

## Version 0.9.0
* Bump MSRV to 1.88.0
* Upgrade to Rust edition 2024
* Add support for multiple time libraries (`time`, `chrono`, and `jiff`), controlled by feature flag
* Add `Ksuid::now(payload)` and `KsuidMs::now(payload)` constructors
* Reject base62 inputs with extra bytes on the end (by @nilium)
* Many internal changes

## Version 0.8.0
* Implement `std::hash::Hash` for Ksuid structs (by @aurelien-clu)

## Version 0.7.0
* Add serde support (and crate feature)
* Implement fmt::Display instead of ToString

## Version 0.6.0
* [SECURITY] Replace the `chrono` crate with `time` (#1)

## Version 0.5.3
* Add homepage to cargo metadata

## Version 0.5.2
* Improve docs and readme

## Version 0.5.1
* Initial open-source release
