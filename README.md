# cgp-serde: Modular Serialization Library for Serde

# Announcement

Read the [announcement blog post for `cgp-serde`](https://contextgeneric.dev/blog/cgp-serde-release/) for the full details.

# Overview

[**cgp-serde**](https://github.com/contextgeneric/cgp-serde) is a modular serialization library for [Serde](https://serde.rs/) that leverages the power of [**Context-Generic Programming**](https://contextgeneric.dev/) (CGP).

In short, `cgp-serde` extends Serde’s original [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) and [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) traits with CGP, making it possible to write **overlapping** or **orphaned** implementations of these traits and thus bypass the standard Rust **coherence restrictions**.

Furthermore, `cgp-serde` allows us to leverage the powerful [**context and capabilities**](https://tmandry.gitlab.io/blog/posts/2021-12-21-context-capabilities/) concepts in stable Rust today. This unlocks the ability to write context-dependent implementations of `Deserialize`, such as one that uses an arena allocator to deserialize a `'a T` value, a concept detailed in the proposal article.


# Context-Generic Serialization Traits

The key highlight of `cgp-serde` is its introduction of context-generic versions of the Serde traits. First, the [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) trait is redefined as follows:

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

Compared to the original `Serialize` trait, `cgp-serde` provides the `CanSerializeValue` CGP trait, which moves the original `Self` type from `Serialize` to an explicit generic parameter named `Value`. The `Self` type in `CanSerializeValue` now represents a **context** type, which can be used for **dependency injection**. The `serialize` method also accepts an extra `&self` value, making it possible to retrieve additional runtime dependencies from this context.

In a similar manner, `cgp-serde` defines a context-generic version of the [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) trait as follows:

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

Analogous to `CanSerializeValue`, the `CanDeserializeValue` trait moves the original `Self` type in `Deserialize` to become the `Value` generic parameter. This `deserialize` method similarly accepts an additional `&self` value, which can be utilized to supply runtime dependencies, such as an arena allocator.

## Provider Traits

In addition to having the extra `Context` parameter as the `Self` type, both `CanSerializeValue` and `CanDeserializeValue` are annotated with the `#[cgp_component]` macro, which is the mechanism that unlocks additional CGP capabilities on these traits.

The `provider` argument to `#[cgp_component]` automatically generates the **provider traits** called `ValueSerializer` and `ValueDeserializer`. These traits are the ones you will use for implementing **named** serialization implementations that can bypass the coherence restrictions.

Conversely, in CGP, we refer to the original traits `CanSerializeValue` and `CanDeserializeValue` as the **consumer traits**. The general rule of thumb is that a CGP component is **used** through its consumer trait but **implemented** using its provider trait.

## `UseDelegate` Provider

Our CGP trait definitions also include a second `derive_delegate` entry within the `#[cgp_component]` macro. This entry generates a specialized `UseDelegate` provider that enables **static dispatch** of provider implementations based on the specific `Value` type.

# Learn More

This repository will be updated with more details later on. Until then, read the [announcement blog post for `cgp-serde`](https://contextgeneric.dev/blog/cgp-serde-release/) for the full details.
