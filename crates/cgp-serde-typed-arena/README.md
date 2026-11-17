[![Crates.io](https://img.shields.io/crates/v/cgp-serde-typed-arena.svg)](https://crates.io/crates/cgp-serde-typed-arena)
[![Docs.rs](https://docs.rs/cgp-serde-typed-arena/badge.svg)](https://docs.rs/cgp-serde-typed-arena)

# cgp-serde-typed-arena: an arena allocator for cgp-serde

`cgp-serde-typed-arena` implements the allocation component of
[`cgp-serde-alloc`](https://docs.rs/cgp-serde-alloc) over `typed_arena::Arena`, so a
[`cgp-serde`](https://docs.rs/cgp-serde) context can deserialize borrowed values into an arena it
holds. cgp-serde rebuilds Serde's traits as components of
[Context-Generic Programming (CGP)](https://contextgeneric.dev/), and the arena is one of the
services a provider can draw from its context while it deserializes.

**[cgp-serde](https://docs.rs/cgp-serde) · [Project page](https://contextgeneric.dev/docs/projects/cgp-serde/) · [Reference](https://contextgeneric.dev/docs/projects/cgp-serde/reference/)**

The crate holds two items:

- **`traits`:** `HasArena<'a, T>`, a getter component that reads a `&'a Arena<T>` from the context,
  with the provider trait `ArenaGetter` and the wiring key `ArenaGetterComponent`. A context wires
  it to the field that holds the arena, typically with `UseField`.
- **`providers`:**
  [`AllocateWithArena`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/allocate_with_arena),
  which implements `CanAlloc` by moving each value into the arena that `HasArena` returns.

A context that deserializes into an arena wires three entries: the arena getter to a field,
`AllocatorComponent` to `AllocateWithArena`, and each borrowed type to `cgp-serde-alloc`'s
`DeserializeAndAllocate`. The arena is an ordinary field borrowed from outside the context, so the
values allocated in it outlive the context and are freed together when the arena is dropped.
