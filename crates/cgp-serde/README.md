[![Crates.io](https://img.shields.io/crates/v/cgp-serde.svg)](https://crates.io/crates/cgp-serde)
[![Docs.rs](https://docs.rs/cgp-serde/badge.svg)](https://docs.rs/cgp-serde)

# cgp-serde: Serde's traits as CGP components

`cgp-serde` defines Serde's `Serialize` and `Deserialize` as components of
[Context-Generic Programming (CGP)](https://contextgeneric.dev/), a language extension for Rust with
pluggable trait implementations at compile-time. Each application is a context type whose wiring
chooses how every type it serializes is encoded, so two applications can write the same value
differently. This crate holds the two components, the adapters that hand a context to Serde, and the
core providers.

**[Project page](https://contextgeneric.dev/docs/projects/cgp-serde/) · [Reference](https://contextgeneric.dev/docs/projects/cgp-serde/reference/) · [Repository](https://github.com/contextgeneric/cgp-serde)**

Here two contexts write the same order, one with its id as a JSON number and one with the id as a
string, as some JavaScript clients need for large ids:

```rust
use cgp::prelude::*;
use cgp_serde::components::{CanSerializeValue, ValueSerializerComponent};
use cgp_serde::providers::{SerializeRecordFields, SerializeWithDisplay, UseSerde};
use cgp_serde::types::SerializeWithContext;

#[derive(CgpData)]
pub struct Order {
    pub id: u64,
    pub item: String,
}

pub struct Api;
pub struct WebClient;

delegate_components! {
    Api {
        open ValueSerializerComponent;
        @ValueSerializerComponent.[u64, String]: UseSerde,
        @ValueSerializerComponent.Order: SerializeRecordFields,
    }
}

delegate_components! {
    WebClient {
        open ValueSerializerComponent;
        @ValueSerializerComponent.u64: SerializeWithDisplay,
        @ValueSerializerComponent.String: UseSerde,
        @ValueSerializerComponent.Order: SerializeRecordFields,
    }
}

check_components! { Api { ValueSerializerComponent: Order } }
check_components! { WebClient { ValueSerializerComponent: Order } }

fn to_json<Context>(context: &Context, order: &Order) -> String
where
    Context: CanSerializeValue<Order>,
{
    serde_json::to_string(&SerializeWithContext::new(context, order)).unwrap()
}

fn main() {
    let order = Order { id: 7, item: "book".to_owned() };

    assert_eq!(to_json(&Api, &order), r#"{"id":7,"item":"book"}"#);
    assert_eq!(to_json(&WebClient, &order), r#"{"id":"7","item":"book"}"#);
}
```

`Order` derives only CGP's general-purpose `CgpData`, and `SerializeRecordFields` writes it as a
map by asking the context how to encode each field. `SerializeWithDisplay` formats the id with its
`Display` impl and asks the context to encode the resulting `String`, which both contexts wire to
`UseSerde`. The `check_components!` lines make the compiler verify each context's whole table.

## What the crate holds

The crate is `no_std` and links `alloc`. Its items sit in three modules:

- **`components`:** `CanSerializeValue<Value>` and `CanDeserializeValue<'de, Value>`, the consumer
  traits callers use, with the provider traits `ValueSerializer` and `ValueDeserializer` and the
  wiring keys `ValueSerializerComponent` and `ValueDeserializerComponent`.
- **`types`:** `SerializeWithContext`, a `Serialize` value, and `DeserializeWithContext`, a
  `DeserializeSeed`, which pass a value and its context to any Serde format.
- **`providers`:** the implementations a context wires for each type.

The providers fall into these families:

- **A type's own Serde impls:** [`UseSerde`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/use_serde).
- **Strings and bytes:** [`SerializeString`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/serialize_string),
  [`SerializeBytes`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/serialize_bytes), and [`TryDeserializeBytes`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/try_deserialize_bytes).
- **Conversions:** [`SerializeWithDisplay`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/serialize_with_display),
  [`DeserializeWithFromStr`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/deserialize_with_from_str), [`SerializeFrom`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/serialize_from),
  [`TrySerializeFrom`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/try_serialize_from), and [`SerializeDeref`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/serialize_deref).
- **Collections:** [`SerializeIterator`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/serialize_iterator) and
  [`DeserializeExtend`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/deserialize_extend).
- **Structs with named fields:** [`SerializeRecordFields`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/serialize_record_fields) and
  [`DeserializeRecordFields`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/deserialize_record_fields).
- **Enums:** [`SerializeVariantFields`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/serialize_variant_fields),
  [`DeserializeVariantFields`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/deserialize_variant_fields), and
  [`SerializeUnit`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/serialize_unit) for a variant with no fields.
- **Defaults:** [`DeserializeDefault`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/deserialize_default), which reads a null as the type's
  default.

Four more crates build on this one: [`cgp-serde-extra`](https://docs.rs/cgp-serde-extra) for byte
and date encodings, [`cgp-serde-json`](https://docs.rs/cgp-serde-json) for JSON operations a
context wires, and [`cgp-serde-alloc`](https://docs.rs/cgp-serde-alloc) and
[`cgp-serde-typed-arena`](https://docs.rs/cgp-serde-typed-arena) for deserializing borrowed values
into an arena. The [project page](https://contextgeneric.dev/docs/projects/cgp-serde/) explains the design and its costs, with a reference page for
each provider.
