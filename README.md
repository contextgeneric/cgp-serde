[![Crates.io](https://img.shields.io/crates/v/cgp-serde.svg)](https://crates.io/crates/cgp-serde)
[![Docs.rs](https://docs.rs/cgp-serde/badge.svg)](https://docs.rs/cgp-serde)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/contextgeneric/cgp-serde/blob/main/LICENSE)
[![Tests](https://github.com/contextgeneric/cgp-serde/actions/workflows/tests.yml/badge.svg)](https://github.com/contextgeneric/cgp-serde/actions/workflows/tests.yml)

# cgp-serde

cgp-serde rebuilds [Serde](https://serde.rs/)'s `Serialize` and `Deserialize` traits as components
of [Context-Generic Programming (CGP)](https://contextgeneric.dev/), a language extension for Rust
with pluggable trait implementations at compile-time. With Serde, a type has one encoding for the
whole program, chosen by whoever owns the type. With cgp-serde, each application chooses how every
type is encoded, so two applications can write the same value differently, and neither needs to own
the type or wrap it in a newtype.

Everything else stays Serde: its data model, its formats such as `serde_json`, and any `Serialize`
impl a type already has.

**[Project page](https://contextgeneric.dev/docs/projects/cgp-serde/) · [Examples](https://contextgeneric.dev/docs/projects/cgp-serde/examples/) · [CGP quickstart](https://contextgeneric.dev/docs/quickstart) · [Announcement post](https://contextgeneric.dev/blog/cgp-serde-release/)**

## One value, two encodings

In this example each application is a **context**, a type that stands for the application and
records its choices in a wiring table. Each entry names the implementation, or **provider**, that
encodes one type. `AppA` writes bytes as hex and `AppB` writes them as base64, and the two tables
differ in that one line:

```rust
use cgp::prelude::*;
use cgp_serde::components::{CanSerializeValue, ValueSerializerComponent};
use cgp_serde::providers::{SerializeRecordFields, UseSerde};
use cgp_serde::types::SerializeWithContext;
use cgp_serde_extra::providers::{SerializeBase64, SerializeHex};

#[derive(CgpData)]
pub struct Message {
    pub id: u64,
    pub data: Vec<u8>,
}

pub struct AppA;
pub struct AppB;

delegate_components! {
    AppA {
        open ValueSerializerComponent;
        @ValueSerializerComponent.[u64, String]: UseSerde,
        @ValueSerializerComponent.Vec<u8>: SerializeHex,
        @ValueSerializerComponent.Message: SerializeRecordFields,
    }
}

delegate_components! {
    AppB {
        open ValueSerializerComponent;
        @ValueSerializerComponent.[u64, String]: UseSerde,
        @ValueSerializerComponent.Vec<u8>: SerializeBase64,
        @ValueSerializerComponent.Message: SerializeRecordFields,
    }
}

check_components! { AppA { ValueSerializerComponent: Message } }
check_components! { AppB { ValueSerializerComponent: Message } }

fn to_json<Context>(context: &Context, message: &Message) -> String
where
    Context: CanSerializeValue<Message>,
{
    serde_json::to_string(&SerializeWithContext::new(context, message)).unwrap()
}

fn main() {
    let message = Message { id: 1, data: b"hi".to_vec() };

    println!("{}", to_json(&AppA, &message)); // {"id":1,"data":"6869"}
    println!("{}", to_json(&AppB, &message)); // {"id":1,"data":"aGk="}
}
```

`Message` derives nothing from Serde, only CGP's general-purpose `CgpData`. `SerializeRecordFields`
writes the struct as a map, asking the context how to encode each field, and the hex and base64
providers ask the same question for the string they produce, which is why each table also wires
`String`. The two `check_components!` lines make the compiler verify that each context can serialize
a whole `Message`, so a missing entry fails the build instead of the program. `SerializeWithContext`
then hands the value and its context to `serde_json` as an ordinary `Serialize` value.

In Serde, `Vec<u8>` has one `Serialize` impl in the whole program, so producing both documents from
the same struct would need a wrapper type for each encoding. The
[`messages`](https://contextgeneric.dev/docs/projects/cgp-serde/examples/messages) example applies
the same idea to a nested archive with dates as well as bytes.

## Installation

cgp-serde builds on stable Rust. Add `cgp`, the core crate, the crates whose providers your wiring
names, and a Serde format:

```toml
[dependencies]
cgp = "0.8.0"
cgp-serde = "0.3.0"
cgp-serde-extra = "0.3.0"
serde_json = "1"
```

## The two components

The library redefines each Serde trait as a CGP component. The type being encoded moves out of
`Self` into a `Value` parameter, and `Self` becomes the context, so the context can choose a
provider for each value type and supply anything a provider needs while it runs:

```rust
#[cgp_component(ValueSerializer)]
#[derive_delegate(UseDelegate<Value>)]
pub trait CanSerializeValue<Value: ?Sized> {
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;
}

#[cgp_component(ValueDeserializer)]
#[derive_delegate(UseDelegate<Value>)]
pub trait CanDeserializeValue<'de, Value> {
    fn deserialize<D>(&self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>;
}
```

`CanSerializeValue` and `CanDeserializeValue` are the **consumer traits** that callers use.
`#[cgp_component]` also generates the **provider traits** `ValueSerializer` and `ValueDeserializer`,
which each provider implements on a type of its own. Implementations that would overlap as blanket
`Serialize` impls can therefore coexist, such as `UseSerde` for any type with a `Serialize` impl and
`SerializeBytes` for any `AsRef<[u8]>`. The `#[derive_delegate]` attribute generates the
`UseDelegate` impl that older code dispatches through with a nested table; new contexts dispatch
with the `open` statement, as the example does.

Because every provider receives the context, a deserializer can also draw a service from it while it
works. The arena crates use this to deserialize borrowed values into an arena that the context
supplies, which Serde's traits have no place to pass.

## The crates

- [`cgp-serde`](./crates/cgp-serde): the two components, the adapters that hand a context to Serde,
  and the core providers for strings, bytes, conversions, collections, structs, and enums.
- [`cgp-serde-extra`](./crates/cgp-serde-extra): encodings for bytes as hex or base64 and for dates
  as RFC 3339 strings or Unix timestamps.
- [`cgp-serde-json`](./crates/cgp-serde-json): JSON serialization and deserialization through
  `serde_json`, as operations a context wires.
- [`cgp-serde-alloc`](./crates/cgp-serde-alloc): the allocation component and the provider that
  deserializes a borrowed value through it.
- [`cgp-serde-typed-arena`](./crates/cgp-serde-typed-arena): an implementation of that component
  over `typed-arena`.

Each external dependency lives in its own crate, so an application compiles only the dependencies
of the providers it wires. Two unpublished crates sit beside them:
[`cgp-serde-examples`](./crates/cgp-serde-examples) holds the runnable examples, and
[`cgp-serde-tests`](./crates/cgp-serde-tests) the test suites.

## Status and costs

cgp-serde is a proof of concept that shows what CGP's per-context wiring does to a trait every Rust
developer knows. It is lightly tested and not meant for production use, and these are the costs to
weigh first:

- **A context names every type it encodes.** The wiring has an entry for each type a value contains,
  including in-between types such as a `Vec` of structs, so it grows longer than the data
  definitions it serves.
- **Mistakes produce long compile errors.** A missing entry deep inside a nested type fails to
  compile, and the compiler's raw message lists every step it took to find it. Checking the context
  with `check_components!` and reading the error with
  [`cargo cgp check`](https://contextgeneric.dev/docs/tooling/cargo-cgp/check) helps:
  `cargo cgp check` leads with the root cause for the classes it recognizes, and the tool does not
  yet reshape every class.
- **Its output suits self-describing formats.** JSON and RON work as they are. Length-prefixed
  binary formats such as postcard reject its structs and sequences, because the providers do not
  declare a length.
- **It covers less than Serde's derive.** The generic providers handle structs with named fields and
  enums whose variants each hold one value or none, written in Serde's externally tagged form. Tuple
  structs, Serde's field attributes such as `rename` and `skip`, and types that contain themselves
  need providers of your own.

When one encoding per type is all a program needs, plain Serde is simpler, and its derive covers
more. The [comparison with Serde](https://contextgeneric.dev/docs/projects/cgp-serde/serde-comparison)
sets out what cgp-serde keeps, adds, and lacks, and the
[limitations page](https://contextgeneric.dev/docs/projects/cgp-serde/limitations) describes where
the design stops.

## Building

These are the commands CI runs, on the stable toolchain that `rust-toolchain.toml` pins:

```sh
cargo build --workspace
cargo test --workspace
```

Each example runs with `cargo run -p cgp-serde-examples --example <name>`. Contributors and agents
start from [AGENTS.md](AGENTS.md), which names these checks.

## Learn more

The [project page](https://contextgeneric.dev/docs/projects/cgp-serde/) explains this library: its
examples as short tutorials, the design one idea at a time, guides to wiring a context and writing a
provider, and a reference page for each provider. For CGP itself, start with the
[quickstart](https://contextgeneric.dev/docs/quickstart), then the
[tutorials](https://contextgeneric.dev/docs/tutorials/hello) and the
[concepts](https://contextgeneric.dev/docs/concepts/). The
[CGP knowledge base](https://github.com/contextgeneric/cgp-knowledge-base) records the project in
depth for contributors and agents.
