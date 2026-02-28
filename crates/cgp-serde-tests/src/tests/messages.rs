use cgp::prelude::*;
use cgp_serde::components::ValueSerializerComponent;
use cgp_serde::providers::{SerializeDeref, SerializeFields, SerializeIterator, UseSerde};
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
        ValueSerializerComponent:
            UseDelegate<new SerializerComponentsA {
                <'a, T> &'a T:
                    SerializeDeref,
                [
                    u64,
                    String,
                ]:
                    UseSerde,
                Vec<u8>:
                    SerializeHex,
                DateTime<Utc>:
                    SerializeRfc3339Date,
                [
                    Vec<EncryptedMessage>,
                    Vec<MessagesByTopic>,
                ]:
                    SerializeIterator,
                [
                    MessagesArchive,
                    MessagesByTopic,
                    EncryptedMessage,
                ]:
                    SerializeFields,
            }>
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
        ValueSerializerComponent:
            UseDelegate<new SerializerComponentsB {
                <'a, T> &'a T:
                    SerializeDeref,
                [
                    i64,
                    u64,
                    String,
                ]:
                    UseSerde,
                Vec<u8>:
                    SerializeBase64,
                DateTime<Utc>:
                    SerializeTimestamp,
                [
                    Vec<EncryptedMessage>,
                    Vec<MessagesByTopic>,
                ]:
                    SerializeIterator,
                [
                    MessagesArchive,
                    MessagesByTopic,
                    EncryptedMessage,
                ]:
                    SerializeFields,
            }>
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

#[test]
fn test_nested_serialization() {
    let archive = MessagesArchive {
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
    };

    let serialized =
        serde_json::to_string_pretty(&SerializeWithContext::new(&AppA, &archive)).unwrap();
    println!("serialized with A: {serialized}");

    let serialized =
        serde_json::to_string_pretty(&SerializeWithContext::new(&AppB, &archive)).unwrap();
    println!("serialized with B: {serialized}");
}
