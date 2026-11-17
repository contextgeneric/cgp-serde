[![Crates.io](https://img.shields.io/crates/v/cgp-serde-alloc.svg)](https://crates.io/crates/cgp-serde-alloc)
[![Docs.rs](https://docs.rs/cgp-serde-alloc/badge.svg)](https://docs.rs/cgp-serde-alloc)

# cgp-serde-alloc: deserialize borrowed values through an allocator

`cgp-serde-alloc` lets [`cgp-serde`](https://docs.rs/cgp-serde), which rebuilds Serde's traits as
components of [Context-Generic Programming (CGP)](https://contextgeneric.dev/), deserialize a
borrowed `&'a T`. It deserializes an owned `T` and moves it into storage that the context
supplies, so the reference outlives the input it was read from. Where that storage lives is a
wiring choice of the context.

**[cgp-serde](https://docs.rs/cgp-serde) · [Project page](https://contextgeneric.dev/docs/projects/cgp-serde/) · [Reference](https://contextgeneric.dev/docs/projects/cgp-serde/reference/)**

The crate holds the two layers that make that choice possible:

- **`traits`:** `CanAlloc<'a, T>`, a component whose method `alloc(&self, value: T) -> &'a mut T`
  moves a value into storage that lives for `'a`, with the provider trait `Allocator` and the
  wiring key `AllocatorComponent`.
- **`providers`:** [`DeserializeAndAllocate`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/deserialize_and_allocate), which deserializes a
  `&'a Value` by deserializing an owned `Value` through the context and allocating it through the
  context's `CanAlloc`.

[`cgp-serde-typed-arena`](https://docs.rs/cgp-serde-typed-arena) implements `CanAlloc` over a
`typed-arena` arena. Another allocator can implement `Allocator` without changing the
deserializer, and this crate adds no dependency beyond `cgp`, `cgp-serde`, and `serde`. The
[context services](https://contextgeneric.dev/docs/projects/cgp-serde/architecture/context-services) page explains how a provider draws a service
from its context.
