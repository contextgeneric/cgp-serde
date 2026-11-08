[![Crates.io](https://img.shields.io/crates/v/cgp-serde.svg)](https://crates.io/crates/cgp-serde)
[![Docs.rs](https://docs.rs/cgp-serde/badge.svg)](https://docs.rs/cgp-serde)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/contextgeneric/cgp-serde/blob/main/LICENSE)
[![Tests](https://github.com/contextgeneric/cgp-serde/actions/workflows/tests.yml/badge.svg)](https://github.com/contextgeneric/cgp-serde/actions/workflows/tests.yml)

# cgp-serde: modular serialization for Serde

**Serde's `Serialize` and `Deserialize` rebuilt as
[Context-Generic Programming (CGP)](https://contextgeneric.dev/) components, so implementations that
Rust's coherence rules reject can coexist.**

`cgp-serde` extends Serde's two traits with CGP, which makes it possible to write overlapping or
orphaned implementations of them and to choose one per context. It also brings context-dependent
deserialization to stable Rust, such as an implementation that deserializes a `'a T` value through an
arena allocator.

**[Website](https://contextgeneric.dev/) · [Project page](https://contextgeneric.dev/docs/projects/cgp-serde/) · [CGP quickstart](https://contextgeneric.dev/docs/quickstart) · [Announcement post](https://contextgeneric.dev/blog/cgp-serde-release/)**

## The two components

The crate redefines each Serde trait as a CGP component. The original `Self` type moves into a
`Value` parameter, and `Self` becomes the context, which supplies runtime dependencies.

```rust
#[cgp_component {
    provider: ValueSerializer,
    derive_delegate: UseDelegate<Value>,
}]
pub trait CanSerializeValue<Value: ?Sized> {
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;
}
```

```rust
#[cgp_component {
    provider: ValueDeserializer,
    derive_delegate: UseDelegate<Value>,
}]
pub trait CanDeserializeValue<'de, Value> {
    fn deserialize<D>(&self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>;
}
```

`CanSerializeValue` and `CanDeserializeValue` are the **consumer traits**, the ones callers use. The
`provider` argument generates the **provider traits** `ValueSerializer` and `ValueDeserializer`, which
named implementations use to bypass the coherence restrictions. A CGP component is used through its
consumer trait and implemented through its provider trait.

The `derive_delegate` argument generates a `UseDelegate` provider that dispatches statically on the
`Value` type. The CGP reference documents the newer `open` statement, which supersedes that idiom for
new code; this crate still uses the generated table.

## The crates

- [`cgp-serde`](./crates/cgp-serde): the two components and their core providers.
- [`cgp-serde-extra`](./crates/cgp-serde-extra): additional providers for serialization and
  deserialization.
- [`cgp-serde-json`](./crates/cgp-serde-json): a CGP wrapper around `serde_json`.
- [`cgp-serde-alloc`](./crates/cgp-serde-alloc) and
  [`cgp-serde-typed-arena`](./crates/cgp-serde-typed-arena): arena-based deserialization.
- [`cgp-serde-examples`](./crates/cgp-serde-examples): runnable examples.

## Building

The workspace builds against the `cgp` repository's `main` branch through its `[patch.crates-io]`
section, because it tracks unreleased CGP code. To build against a local `cgp` checkout, switch those
entries to the commented-out paths into `../cgp`.

```sh
cargo build --workspace
cargo test --workspace
```

## Learn CGP

The website is the main documentation for CGP: the
[quickstart](https://contextgeneric.dev/docs/quickstart), the
[tutorials](https://contextgeneric.dev/docs/tutorials/hello), the
[concepts](https://contextgeneric.dev/docs/concepts/), and the
[construct reference](https://contextgeneric.dev/docs/reference/). The
[project page](https://contextgeneric.dev/docs/projects/cgp-serde/) explains this repository, and the
[CGP knowledge base](https://github.com/contextgeneric/cgp-knowledge-base) records it in depth for
contributors and agents.

## License

Licensed under the MIT license.
