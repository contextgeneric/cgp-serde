//! Runnable examples for the cgp-serde crates.
//!
//! The crate has no library code. Each example under `examples/` wires one scenario end to end:
//!
//! - `basic`: one struct round-tripped through JSON, with its bytes as hex.
//! - `messages`: one archive serialized by two applications that encode bytes and dates
//!   differently.
//! - `events`: a batch of chat events, an enum whose variants hold records, written and read back
//!   by two applications that encode bytes and dates differently.
//! - `arena_simplified`: borrowed values deserialized into an arena the context supplies, with a
//!   local getter and deserializer.
//! - `arena`: the same through the layered allocation crates, where the allocator is a wiring
//!   choice.
//!
//! Run one with `cargo run --example <name>`. `cargo test` also runs each example's assertions.
