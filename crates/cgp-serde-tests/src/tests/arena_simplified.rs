use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::handler::CanTryCompute;
use cgp::prelude::*;
use cgp_error_anyhow::{RaiseAnyhowError, UseAnyhowError};
use cgp_serde::components::{CanDeserializeValue, ValueDeserializer, ValueDeserializerComponent};
use cgp_serde::providers::{DeserializeExtend, DeserializeRecordFields, UseSerde};
use cgp_serde_json::code::{DeserializeJson, SerializeJson};
use cgp_serde_json::{DeserializeFromJsonString, SerializeToJsonString};
use typed_arena::Arena;

#[cgp_auto_getter]
pub trait HasArena<'a, T: 'a> {
    fn arena(&self) -> &&'a Arena<T>;
}

#[cgp_impl(new DeserializeAndAllocate)]
impl<'de, 'a, Context, Value> ValueDeserializer<'de, &'a Value> for Context
where
    Context: HasArena<'a, Value> + CanDeserializeValue<'de, Value>,
{
    fn deserialize<D>(context: &Context, deserializer: D) -> Result<&'a Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = context.deserialize(deserializer)?;
        let value = context.arena().alloc(value);

        Ok(value)
    }
}

#[derive(Debug, PartialEq, Eq, HasFields, BuildField)]
pub struct Coord {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

#[derive(Debug, PartialEq, Eq, HasFields, BuildField)]
pub struct Payload<'a> {
    pub id: u64,
    pub coords: Vec<&'a Coord>,
}

#[derive(HasField)]
pub struct App<'a> {
    pub arena: &'a Arena<Coord>,
}

delegate_components! {
    <'a> App<'a> {
        ErrorTypeProviderComponent:
            UseAnyhowError,
        ErrorRaiserComponent:
            RaiseAnyhowError,
        ValueDeserializerComponent:
            UseDelegate<new DeserializeComponents {
                u64: UseSerde,
                Coord:
                    DeserializeRecordFields,
                <'a> &'a Coord:
                    DeserializeAndAllocate,
                <'a> Vec<&'a Coord>:
                    DeserializeExtend,
                <'a> Payload<'a>:
                    DeserializeRecordFields,

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
    <'a> CanUseApp for App<'a> {
        ValueDeserializerComponent:
            (Life<'a>, Coord),

    }
}

check_components! {
    <'de, 'a> CanDeserializeApp for App<'a> {
        ValueDeserializerComponent: [
            (Life<'de>, u64),
            (Life<'de>, Coord),
            (Life<'de>, &'a Coord),
            (Life<'de>, Payload<'a>),
        ]
    }
}

#[test]
fn test_deserialize_with_arena() {
    let serialized = r#"
{
    "id": 8,
    "coords": [
        { "x": 1, "y": 2, "z": 3 },
        { "x": 4, "y": 5, "z": 6 }
    ]
}
"#;

    let arena = Arena::new();
    let app = App { arena: &arena };

    let deserialized: Payload<'_> = app
        .try_compute(PhantomData::<DeserializeJson<Payload<'_>>>, &serialized)
        .unwrap();
    assert_eq!(
        deserialized,
        Payload {
            id: 8,
            coords: vec![&Coord { x: 1, y: 2, z: 3 }, &Coord { x: 4, y: 5, z: 6 },]
        }
    );
}
