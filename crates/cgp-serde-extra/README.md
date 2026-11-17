[![Crates.io](https://img.shields.io/crates/v/cgp-serde-extra.svg)](https://crates.io/crates/cgp-serde-extra)
[![Docs.rs](https://docs.rs/cgp-serde-extra/badge.svg)](https://docs.rs/cgp-serde-extra)

# cgp-serde-extra: byte and date encodings for cgp-serde

`cgp-serde-extra` adds four encodings to [`cgp-serde`](https://docs.rs/cgp-serde), which rebuilds
Serde's traits as components of [Context-Generic Programming (CGP)](https://contextgeneric.dev/).
Each encoding is a provider that a context wires for the type it encodes, so one application can
write bytes as hex while another writes them as base64, from the same struct.

**[cgp-serde](https://docs.rs/cgp-serde) · [Project page](https://contextgeneric.dev/docs/projects/cgp-serde/) · [Reference](https://contextgeneric.dev/docs/projects/cgp-serde/reference/)**

| Provider | Encodes | Writes |
| --- | --- | --- |
| [`SerializeHex`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/serialize_hex) | a `ToHex` value such as `Vec<u8>` | lowercase hexadecimal: `b"hi"` is `"6869"` |
| [`SerializeBase64`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/serialize_base64) | any `AsRef<[u8]>`, read back as `Vec<u8>` | standard padded base64: `b"hi"` is `"aGk="` |
| [`SerializeRfc3339Date`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/serialize_rfc3339_date) | `DateTime<Utc>` | an RFC 3339 string such as `"2025-11-03T14:15:00+00:00"` |
| [`SerializeTimestamp`](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/serialize_timestamp) | `DateTime<Utc>` | whole seconds since the Unix epoch, such as `1762179300` |

Each provider both serializes and deserializes. It converts the value to a `String`, or an `i64` for
the timestamp, and encodes that through the context, so a context that wires one of them also wires
the intermediate type, usually to `UseSerde`. A timestamp keeps no sub-second part, so a value with
milliseconds does not round-trip exactly.

The providers are in `cgp_serde_extra::providers`, and the crate depends on `hex`, `base64`, and
`chrono`. The [`messages`](https://contextgeneric.dev/docs/projects/cgp-serde/examples/messages)
example wires two applications that differ by these providers.
