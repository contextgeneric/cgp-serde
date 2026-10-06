use cgp::prelude::*;
use cgp_serde::components::{
    CanDeserializeValue, CanSerializeValue, ValueDeserializerComponent, ValueSerializerComponent,
};
use cgp_serde::providers::{
    DeserializeExtend, DeserializeRecordFields, DeserializeVariantFields, SerializeDeref,
    SerializeIterator, SerializeRecordFields, SerializeUnit, SerializeVariantFields, UseSerde,
};
use cgp_serde::types::{DeserializeWithContext, SerializeWithContext};
use cgp_serde_extra::providers::{
    SerializeBase64, SerializeHex, SerializeRfc3339Date, SerializeTimestamp,
};
use chrono::{DateTime, TimeZone, Utc};
use serde::de::DeserializeSeed;

#[derive(Debug, PartialEq, CgpData)]
pub struct Posted {
    pub message_id: u64,
    pub author_id: u64,
    pub date: DateTime<Utc>,
    pub encrypted_data: Vec<u8>,
}

#[derive(Debug, PartialEq, CgpData)]
pub struct Edited {
    pub message_id: u64,
    pub date: DateTime<Utc>,
    pub encrypted_data: Vec<u8>,
}

#[derive(Debug, PartialEq, CgpData)]
pub struct Reacted {
    pub message_id: u64,
    pub author_id: u64,
    pub emoji: String,
}

#[derive(Debug, PartialEq, CgpVariant)]
pub enum ChatEvent {
    Posted(Posted),
    Edited(Edited),
    Reacted(Reacted),
    HistoryCleared,
}

#[derive(Debug, PartialEq, CgpData)]
pub struct SyncBatch {
    pub device_key: Vec<u8>,
    pub events: Vec<ChatEvent>,
}

/// Stores batches compactly: bytes as base64, dates as Unix timestamps.
pub struct ServerApp;

delegate_components! {
    ServerApp {
        open {
            ValueSerializerComponent,
            ValueDeserializerComponent,
        };

        @ValueSerializerComponent.<'a, T> &'a T:
            SerializeDeref,
        @ValueSerializerComponent.[i64, u64, String]:
            UseSerde,
        @ValueSerializerComponent.Nil:
            SerializeUnit,
        @ValueSerializerComponent.Vec<u8>:
            SerializeBase64,
        @ValueSerializerComponent.DateTime<Utc>:
            SerializeTimestamp,
        @ValueSerializerComponent.[Posted, Edited, Reacted, SyncBatch]:
            SerializeRecordFields,
        @ValueSerializerComponent.ChatEvent:
            SerializeVariantFields,
        @ValueSerializerComponent.Vec<ChatEvent>:
            SerializeIterator,

        @ValueDeserializerComponent.[i64, u64, String]:
            UseSerde,
        @ValueDeserializerComponent.Nil:
            SerializeUnit,
        @ValueDeserializerComponent.Vec<u8>:
            SerializeBase64,
        @ValueDeserializerComponent.DateTime<Utc>:
            SerializeTimestamp,
        @ValueDeserializerComponent.[Posted, Edited, Reacted, SyncBatch]:
            DeserializeRecordFields,
        @ValueDeserializerComponent.ChatEvent:
            DeserializeVariantFields,
        @ValueDeserializerComponent.Vec<ChatEvent>:
            DeserializeExtend,
    }
}

check_components! {
    #[check_trait(CanSerializeOnServer)]
    ServerApp {
        ValueSerializerComponent: [
            i64,
            u64,
            String,
            Nil,
            Vec<u8>,
            DateTime<Utc>,
            Posted,
            Edited,
            Reacted,
            ChatEvent,
            Vec<ChatEvent>,
            SyncBatch,
        ],
    }
}

check_components! {
    #[check_trait(CanDeserializeOnServer)]
    <'de> ServerApp {
        ValueDeserializerComponent: [
            (Life<'de>, i64),
            (Life<'de>, u64),
            (Life<'de>, String),
            (Life<'de>, Nil),
            (Life<'de>, Vec<u8>),
            (Life<'de>, DateTime<Utc>),
            (Life<'de>, Posted),
            (Life<'de>, Edited),
            (Life<'de>, Reacted),
            (Life<'de>, ChatEvent),
            (Life<'de>, Vec<ChatEvent>),
            (Life<'de>, SyncBatch),
        ],
    }
}

/// Writes batches for people to read: bytes as hex, dates as RFC 3339 strings.
pub struct InspectorApp;

delegate_components! {
    InspectorApp {
        open {
            ValueSerializerComponent,
            ValueDeserializerComponent,
        };

        @ValueSerializerComponent.<'a, T> &'a T:
            SerializeDeref,
        @ValueSerializerComponent.[u64, String]:
            UseSerde,
        @ValueSerializerComponent.Nil:
            SerializeUnit,
        @ValueSerializerComponent.Vec<u8>:
            SerializeHex,
        @ValueSerializerComponent.DateTime<Utc>:
            SerializeRfc3339Date,
        @ValueSerializerComponent.[Posted, Edited, Reacted, SyncBatch]:
            SerializeRecordFields,
        @ValueSerializerComponent.ChatEvent:
            SerializeVariantFields,
        @ValueSerializerComponent.Vec<ChatEvent>:
            SerializeIterator,

        @ValueDeserializerComponent.[u64, String]:
            UseSerde,
        @ValueDeserializerComponent.Nil:
            SerializeUnit,
        @ValueDeserializerComponent.Vec<u8>:
            SerializeHex,
        @ValueDeserializerComponent.DateTime<Utc>:
            SerializeRfc3339Date,
        @ValueDeserializerComponent.[Posted, Edited, Reacted, SyncBatch]:
            DeserializeRecordFields,
        @ValueDeserializerComponent.ChatEvent:
            DeserializeVariantFields,
        @ValueDeserializerComponent.Vec<ChatEvent>:
            DeserializeExtend,
    }
}

check_components! {
    #[check_trait(CanSerializeInInspector)]
    InspectorApp {
        ValueSerializerComponent: [
            u64,
            String,
            Nil,
            Vec<u8>,
            DateTime<Utc>,
            Posted,
            Edited,
            Reacted,
            ChatEvent,
            Vec<ChatEvent>,
            SyncBatch,
        ],
    }
}

check_components! {
    #[check_trait(CanDeserializeInInspector)]
    <'de> InspectorApp {
        ValueDeserializerComponent: [
            (Life<'de>, u64),
            (Life<'de>, String),
            (Life<'de>, Nil),
            (Life<'de>, Vec<u8>),
            (Life<'de>, DateTime<Utc>),
            (Life<'de>, Posted),
            (Life<'de>, Edited),
            (Life<'de>, Reacted),
            (Life<'de>, ChatEvent),
            (Life<'de>, Vec<ChatEvent>),
            (Life<'de>, SyncBatch),
        ],
    }
}

/// One batch with every kind of event. The dates are whole seconds, because Unix timestamps drop
/// anything finer.
fn batch() -> SyncBatch {
    SyncBatch {
        device_key: b"device-7".into(),
        events: vec![
            ChatEvent::Posted(Posted {
                message_id: 1,
                author_id: 2,
                date: Utc.with_ymd_and_hms(2025, 11, 3, 14, 15, 0).unwrap(),
                encrypted_data: b"Hello".into(),
            }),
            ChatEvent::Edited(Edited {
                message_id: 1,
                date: Utc.with_ymd_and_hms(2025, 11, 3, 14, 16, 0).unwrap(),
                encrypted_data: b"Hello!".into(),
            }),
            ChatEvent::Reacted(Reacted {
                message_id: 1,
                author_id: 3,
                emoji: "👍🏽".into(),
            }),
            ChatEvent::HistoryCleared,
        ],
    }
}

/// Serializes `batch` as pretty-printed JSON through `context`.
fn to_json<Context>(context: &Context, batch: &SyncBatch) -> String
where
    Context: CanSerializeValue<SyncBatch>,
{
    serde_json::to_string_pretty(&SerializeWithContext::new(context, batch)).unwrap()
}

/// Deserializes a batch from JSON through `context`, rejecting trailing input.
fn from_json<Context>(context: &Context, json: &str) -> Result<SyncBatch, serde_json::Error>
where
    Context: for<'de> CanDeserializeValue<'de, SyncBatch>,
{
    let mut deserializer = serde_json::Deserializer::from_str(json);
    let batch = DeserializeWithContext::new(context).deserialize(&mut deserializer)?;
    deserializer.end()?;
    Ok(batch)
}

fn main() {
    let batch = batch();

    let server_json = to_json(&ServerApp, &batch);
    let inspector_json = to_json(&InspectorApp, &batch);

    println!("written by the server:\n{server_json}\n");
    println!("written by the inspector:\n{inspector_json}\n");

    println!(
        "each context reads its own JSON back: {}",
        from_json(&ServerApp, &server_json).unwrap() == batch
            && from_json(&InspectorApp, &inspector_json).unwrap() == batch
    );

    println!(
        "the inspector reading the server's JSON: {}",
        from_json(&InspectorApp, &server_json).unwrap_err()
    );
    println!(
        "the server reading the inspector's JSON: {}",
        from_json(&ServerApp, &inspector_json).unwrap_err()
    );
}

#[test]
fn test_sync_events() {
    let batch = batch();

    let server_json = to_json(&ServerApp, &batch);
    let inspector_json = to_json(&InspectorApp, &batch);

    assert_eq!(
        server_json,
        r#"{
  "device_key": "ZGV2aWNlLTc=",
  "events": [
    {
      "Posted": {
        "message_id": 1,
        "author_id": 2,
        "date": 1762179300,
        "encrypted_data": "SGVsbG8="
      }
    },
    {
      "Edited": {
        "message_id": 1,
        "date": 1762179360,
        "encrypted_data": "SGVsbG8h"
      }
    },
    {
      "Reacted": {
        "message_id": 1,
        "author_id": 3,
        "emoji": "👍🏽"
      }
    },
    {
      "HistoryCleared": null
    }
  ]
}"#
    );

    assert_eq!(
        inspector_json,
        r#"{
  "device_key": "6465766963652d37",
  "events": [
    {
      "Posted": {
        "message_id": 1,
        "author_id": 2,
        "date": "2025-11-03T14:15:00+00:00",
        "encrypted_data": "48656c6c6f"
      }
    },
    {
      "Edited": {
        "message_id": 1,
        "date": "2025-11-03T14:16:00+00:00",
        "encrypted_data": "48656c6c6f21"
      }
    },
    {
      "Reacted": {
        "message_id": 1,
        "author_id": 3,
        "emoji": "👍🏽"
      }
    },
    {
      "HistoryCleared": null
    }
  ]
}"#
    );

    assert_eq!(from_json(&ServerApp, &server_json).unwrap(), batch);
    assert_eq!(from_json(&InspectorApp, &inspector_json).unwrap(), batch);

    // Hex cannot decode base64, so the inspector fails on the first field.
    assert_eq!(
        from_json(&InspectorApp, &server_json)
            .unwrap_err()
            .to_string(),
        "Invalid character 'Z' at position 0 at line 2 column 30"
    );

    // The hex device key is also valid base64, so the server misreads it silently and fails only
    // at the first date.
    assert_eq!(
        from_json(&ServerApp, &inspector_json)
            .unwrap_err()
            .to_string(),
        "invalid type: string \"2025-11-03T14:15:00+00:00\", expected i64 at line 8 column 43"
    );
}
