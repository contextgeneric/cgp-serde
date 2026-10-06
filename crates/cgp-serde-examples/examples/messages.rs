use cgp::prelude::*;
use cgp_serde::components::{CanSerializeValue, ValueSerializerComponent};
use cgp_serde::providers::{SerializeDeref, SerializeIterator, SerializeRecordFields, UseSerde};
use cgp_serde::types::SerializeWithContext;
use cgp_serde_extra::providers::{
    SerializeBase64, SerializeHex, SerializeRfc3339Date, SerializeTimestamp,
};
use chrono::{DateTime, TimeZone, Utc};

#[derive(CgpData)]
pub struct EncryptedMessage {
    pub message_id: u64,
    pub author_id: u64,
    pub date: DateTime<Utc>,
    pub encrypted_data: Vec<u8>,
}

#[derive(CgpData)]
pub struct MessagesByTopic {
    pub encrypted_topic: Vec<u8>,
    pub messages: Vec<EncryptedMessage>,
}

#[derive(CgpData)]
pub struct MessagesArchive {
    pub decryption_key: Vec<u8>,
    pub messages_by_topics: Vec<MessagesByTopic>,
}

pub struct AppA;

delegate_components! {
    AppA {
        open {ValueSerializerComponent};


        @ValueSerializerComponent.<'a, T> &'a T:
            SerializeDeref,

        @ValueSerializerComponent.[
            u64,
            String,
        ]:
            UseSerde,

        @ValueSerializerComponent.Vec<u8>:
            SerializeHex,

        @ValueSerializerComponent.DateTime<Utc>:
            SerializeRfc3339Date,

        @ValueSerializerComponent.[
            Vec<EncryptedMessage>,
            Vec<MessagesByTopic>,
        ]:
            SerializeIterator,

        @ValueSerializerComponent.[
            MessagesArchive,
            MessagesByTopic,
            EncryptedMessage,
        ]:
            SerializeRecordFields,
    }
}

check_components! {
    AppA {
        ValueSerializerComponent: [
            u64,
            String,
            Vec<u8>,
            DateTime<Utc>,
            EncryptedMessage,
            MessagesByTopic,
            MessagesArchive,
        ]
    }
}

pub struct AppB;

delegate_components! {
    AppB {
        open {ValueSerializerComponent};

        @ValueSerializerComponent.<'a, T> &'a T:
            SerializeDeref,

        @ValueSerializerComponent.[
            i64,
            u64,
            String,
        ]:
            UseSerde,

        @ValueSerializerComponent.Vec<u8>:
            SerializeBase64,

        @ValueSerializerComponent.DateTime<Utc>:
            SerializeTimestamp,

        @ValueSerializerComponent.[
            Vec<EncryptedMessage>,
            Vec<MessagesByTopic>,
        ]:
            SerializeIterator,

        @ValueSerializerComponent.[
            MessagesArchive,
            MessagesByTopic,
            EncryptedMessage,
        ]:
            SerializeRecordFields,
    }
}

check_components! {
    AppB {
        ValueSerializerComponent: [
            u64,
            String,
            Vec<u8>,
            DateTime<Utc>,
            EncryptedMessage,
            MessagesByTopic,
            MessagesArchive,
        ]
    }
}

fn archive() -> MessagesArchive {
    MessagesArchive {
        decryption_key: b"top-secret".into(),
        messages_by_topics: vec![MessagesByTopic {
            encrypted_topic: b"All about CGP".into(),
            messages: vec![
                EncryptedMessage {
                    message_id: 1,
                    author_id: 2,
                    date: Utc.with_ymd_and_hms(2025, 11, 3, 14, 15, 0).unwrap(),
                    encrypted_data: b"Hello from RustLab!".into(),
                },
                EncryptedMessage {
                    message_id: 4,
                    author_id: 8,
                    date: Utc.with_ymd_and_hms(2025, 12, 19, 23, 45, 0).unwrap(),
                    encrypted_data: b"One year anniversary!".into(),
                },
            ],
        }],
    }
}

/// Serializes `archive` as pretty-printed JSON through `context`.
fn to_pretty_json<Context>(context: &Context, archive: &MessagesArchive) -> String
where
    Context: CanSerializeValue<MessagesArchive>,
{
    serde_json::to_string_pretty(&SerializeWithContext::new(context, archive)).unwrap()
}

fn main() {
    let archive = archive();

    println!("serialized with A: {}", to_pretty_json(&AppA, &archive));
    println!("serialized with B: {}", to_pretty_json(&AppB, &archive));
}

#[test]
fn test_nested_serialization() {
    let archive = archive();

    let serialized_a = to_pretty_json(&AppA, &archive);

    assert_eq!(
        serialized_a,
        r#"{
  "decryption_key": "746f702d736563726574",
  "messages_by_topics": [
    {
      "encrypted_topic": "416c6c2061626f757420434750",
      "messages": [
        {
          "message_id": 1,
          "author_id": 2,
          "date": "2025-11-03T14:15:00+00:00",
          "encrypted_data": "48656c6c6f2066726f6d20527573744c616221"
        },
        {
          "message_id": 4,
          "author_id": 8,
          "date": "2025-12-19T23:45:00+00:00",
          "encrypted_data": "4f6e65207965617220616e6e697665727361727921"
        }
      ]
    }
  ]
}"#
    );

    let serialized_b = to_pretty_json(&AppB, &archive);

    assert_eq!(
        serialized_b,
        r#"{
  "decryption_key": "dG9wLXNlY3JldA==",
  "messages_by_topics": [
    {
      "encrypted_topic": "QWxsIGFib3V0IENHUA==",
      "messages": [
        {
          "message_id": 1,
          "author_id": 2,
          "date": 1762179300,
          "encrypted_data": "SGVsbG8gZnJvbSBSdXN0TGFiIQ=="
        },
        {
          "message_id": 4,
          "author_id": 8,
          "date": 1766187900,
          "encrypted_data": "T25lIHllYXIgYW5uaXZlcnNhcnkh"
        }
      ]
    }
  ]
}"#
    );
}
