use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;
use cgp_error_anyhow::{RaiseAnyhowError, UseAnyhowError};
use cgp_serde::components::{CanDeserializeValue, ValueDeserializer, ValueDeserializerComponent};
use cgp_serde::providers::{DeserializeExtend, DeserializeRecordFields, UseSerde};
use cgp_serde_json::impls::CanDeserializeJsonString;
use typed_arena::Arena;

#[cgp_auto_getter]
pub trait HasArena<'a, T: 'a> {
    fn arena(&self) -> &&'a Arena<T>;
}

#[cgp_impl(new DeserializeAndAllocate)]
#[uses(HasArena<'a, Value>, CanDeserializeValue<'de, Value>)]
impl<'de, 'a, Value> ValueDeserializer<'de, &'a Value> {
    fn deserialize<D>(&self, deserializer: D) -> Result<&'a Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = self.deserialize(deserializer)?;
        let value = self.arena().alloc(value);

        Ok(value)
    }
}

#[derive(Debug, PartialEq, Eq, CgpData)]
pub struct Coord {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

#[derive(Debug, PartialEq, Eq, CgpData)]
pub struct Cluster<'a> {
    pub id: u64,
    pub coords: Vec<&'a Coord>,
}

#[derive(HasField)]
pub struct App<'a> {
    pub arena: &'a Arena<Coord>,
}

delegate_components! {
    <'s> App<'s> {
        ValueDeserializerComponent:
            UseDelegate<new DeserializeComponents {
                u64: UseSerde,
                [
                    Coord,
                    <'a> Cluster<'a>,
                ]:
                    DeserializeRecordFields,
                <'a> &'a Coord:
                    DeserializeAndAllocate,
                <'a> Vec<&'a Coord>:
                    DeserializeExtend,

            }>,
        ErrorTypeProviderComponent:
            UseAnyhowError,
        ErrorRaiserComponent:
            RaiseAnyhowError,
    }
}

check_components! {
    #[check_trait(CanUseApp)]
    <'a> App<'a> {
        ValueDeserializerComponent:
            (Life<'a>, Coord),

    }
}

check_components! {
    #[check_trait(CanDeserializeApp)]
    <'de, 'a> App<'a> {
        ValueDeserializerComponent: [
            (Life<'de>, u64),
            (Life<'de>, Coord),
            (Life<'de>, &'a Coord),
            (Life<'de>, Cluster<'a>),
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

    let deserialized: Cluster<'_> = app.deserialize_json_string(&serialized).unwrap();

    assert_eq!(
        deserialized,
        Cluster {
            id: 8,
            coords: vec![&Coord { x: 1, y: 2, z: 3 }, &Coord { x: 4, y: 5, z: 6 },]
        }
    );
}
