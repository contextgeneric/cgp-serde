use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::handler::CanTryCompute;
use cgp::prelude::*;
use cgp_error_anyhow::{RaiseAnyhowError, UseAnyhowError};
use cgp_serde::components::{ValueDeserializerComponent, ValueSerializerComponent};
use cgp_serde::providers::{DeserializeRecordFields, SerializeFields, SerializeString, UseSerde};
use cgp_serde_extra::providers::SerializeHex;
use cgp_serde_json::code::{DeserializeJson, SerializeJson};
use cgp_serde_json::providers::{DeserializeFromJsonString, SerializeToJsonString};

#[derive(Debug, Eq, PartialEq, CgpData)]
pub struct Payload {
    pub quantity: u64,
    pub message: String,
    pub data: Vec<u8>,
}

pub struct App;

delegate_components! {
    App {
        ErrorTypeProviderComponent:
            UseAnyhowError,
        ErrorRaiserComponent:
            RaiseAnyhowError,
        ValueSerializerComponent:
            UseDelegate<new SerializerComponents {
                u64:
                    UseSerde,
                String:
                    SerializeString,
                Vec<u8>:
                    SerializeHex,
                Payload:
                    SerializeFields,
            }>,
        ValueDeserializerComponent:
            UseDelegate<new DeserializerComponents {
                [
                    u64,
                    String,
                ]:
                    UseSerde,
                Payload:
                    DeserializeRecordFields,
                Vec<u8>: SerializeHex,
            }>,
        TryComputerComponent:
            UseDelegate<new JsonEncodingComponents {
                SerializeJson:
                    SerializeToJsonString,
                <T> DeserializeJson<T>:
                    DeserializeFromJsonString
            }>,
    }
}

check_components! {
    #[check_trait(CanUseAppSerializer)]
    App {
        ValueSerializerComponent: [
            u64,
            String,
            Vec<u8>,
            Payload,
        ],
    }
}

check_components! {
    #[check_trait(CanDeserializeApp)]
    <'de> App {
        ValueDeserializerComponent: [
            (Life<'de>, u64),
            (Life<'de>, String),
            (Life<'de>, Vec<u8>),
            (Life<'de>, Payload),
        ]
    }
}

#[test]
fn test_basic_serialization() {
    let context = App;

    let value = Payload {
        quantity: 42,
        message: "hello".to_owned(),
        data: vec![1, 2, 3],
    };

    let serialized = context
        .try_compute(PhantomData::<SerializeJson>, &value)
        .unwrap();

    assert_eq!(
        serialized,
        "{\"quantity\":42,\"message\":\"hello\",\"data\":\"010203\"}"
    );

    let deserialized: Payload = context
        .try_compute(PhantomData::<DeserializeJson<Payload>>, &serialized)
        .unwrap();

    assert_eq!(deserialized, value);
}
