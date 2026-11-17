[![Crates.io](https://img.shields.io/crates/v/cgp-serde-json.svg)](https://crates.io/crates/cgp-serde-json)
[![Docs.rs](https://docs.rs/cgp-serde-json/badge.svg)](https://docs.rs/cgp-serde-json)

# cgp-serde-json: JSON operations for cgp-serde contexts

`cgp-serde-json` connects [`cgp-serde`](https://docs.rs/cgp-serde), which rebuilds Serde's traits
as components of [Context-Generic Programming (CGP)](https://contextgeneric.dev/), to
`serde_json`. Encoding to JSON and decoding from it become fallible operations on CGP's
`TryComputer` handler, which a context wires like any other component, and each `serde_json` error
is raised into the context's own error type.

**[cgp-serde](https://docs.rs/cgp-serde) · [Project page](https://contextgeneric.dev/docs/projects/cgp-serde/) · [Reference](https://contextgeneric.dev/docs/projects/cgp-serde/reference/)**

The crate is `no_std` and links `alloc`. Its items sit in three modules:

- **`code`:** `SerializeJson` and `DeserializeJson<T>`, the types that key a context's
  `TryComputerComponent` entries for the two directions.
- **`providers`:** [`SerializeToJsonString`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/serialize_to_json_string), which writes compact
  JSON; [`DeserializeFromJsonReader`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/deserialize_from_json_reader), which reads from any
  `serde_json` reader and rejects trailing input; and
  [`DeserializeFromJsonString`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/deserialize_from_json_string), which reads from any
  `AsRef<str>` through the reader provider.
- **`impls`:** `CanDeserializeJsonString<T>`, which gives a context with value deserializers and
  error wiring a `deserialize_json_string` method.

The providers are optional. Any context can already serialize with `serde_json::to_string` and a
`SerializeWithContext` from `cgp-serde`; the providers add the error conversion, the end-of-input
check, and a place in the wiring. The [`basic`](https://contextgeneric.dev/docs/projects/cgp-serde/examples/basic) example round-trips a struct
through them.
