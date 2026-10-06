//! A unit-like variant holds `()`, and the context's wiring for `()` decides how it is written.

use core::fmt;

use cgp::prelude::*;
use cgp_serde::components::{
    ValueDeserializer, ValueDeserializerComponent, ValueSerializer, ValueSerializerComponent,
};
use cgp_serde::providers::{
    DeserializeRecordFields, DeserializeVariantFields, SerializeRecordFields,
    SerializeVariantFields, UseSerde,
};
use serde::de::{Error, IgnoredAny, MapAccess, Visitor};
use serde::ser::SerializeMap;

use super::{App, Circle, Rectangle, Shape};
use crate::tests::support::{assert_json_round_trip, from_json};

/// Writes `()` as an empty map, `{}`, and reads only an empty map back.
pub struct UnitAsEmptyMap;

#[cgp_impl(UnitAsEmptyMap)]
impl ValueSerializer<()> {
    fn serialize<S>(&self, _value: &(), serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_map(Some(0))?.end()
    }
}

#[cgp_impl(UnitAsEmptyMap)]
impl<'de> ValueDeserializer<'de, ()> {
    fn deserialize<D>(&self, deserializer: D) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(EmptyMapVisitor)
    }
}

struct EmptyMapVisitor;

impl<'de> Visitor<'de> for EmptyMapVisitor {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "an empty map")
    }

    fn visit_map<M>(self, mut map: M) -> Result<(), M::Error>
    where
        M: MapAccess<'de>,
    {
        match map.next_key::<IgnoredAny>()? {
            None => Ok(()),
            Some(_) => Err(M::Error::custom("expected an empty map")),
        }
    }
}

pub struct EmptyMapApp;

delegate_components! {
    EmptyMapApp {
        open {
            ValueSerializerComponent,
            ValueDeserializerComponent,
        };

        @ValueSerializerComponent.[u64, String]: UseSerde,
        @ValueSerializerComponent.(): UnitAsEmptyMap,
        @ValueSerializerComponent.[Circle, Rectangle]: SerializeRecordFields,
        @ValueSerializerComponent.Shape: SerializeVariantFields,

        @ValueDeserializerComponent.[u64, String]: UseSerde,
        @ValueDeserializerComponent.(): UnitAsEmptyMap,
        @ValueDeserializerComponent.[Circle, Rectangle]: DeserializeRecordFields,
        @ValueDeserializerComponent.Shape: DeserializeVariantFields,
    }
}

check_components! {
    #[check_trait(CanSerializeWithEmptyMap)]
    EmptyMapApp {
        ValueSerializerComponent: Shape,
    }
}

check_components! {
    #[check_trait(CanDeserializeWithEmptyMap)]
    <'de> EmptyMapApp {
        ValueDeserializerComponent: (Life<'de>, Shape),
    }
}

#[test]
fn unit_through_use_serde_is_null() {
    assert_json_round_trip(&App, Shape::Empty(()), r#"{"Empty":null}"#);
    assert_eq!(
        from_json::<App, Shape>(&App, r#"{"Empty":{}}"#),
        Err("invalid type: map, expected unit at line 1 column 9".to_owned())
    );
}

#[test]
fn unit_through_a_context_provider_is_an_empty_map() {
    assert_json_round_trip(&EmptyMapApp, Shape::Empty(()), r#"{"Empty":{}}"#);
    assert_eq!(
        from_json::<EmptyMapApp, Shape>(&EmptyMapApp, r#"{"Empty":null}"#),
        Err("invalid type: null, expected an empty map at line 1 column 13".to_owned())
    );
}

#[test]
fn a_bare_variant_name_is_rejected_whatever_the_unit_format() {
    assert_eq!(
        from_json::<App, Shape>(&App, r#""Empty""#),
        Err("invalid type: unit variant, expected newtype variant".to_owned())
    );
    assert_eq!(
        from_json::<EmptyMapApp, Shape>(&EmptyMapApp, r#""Empty""#),
        Err("invalid type: unit variant, expected newtype variant".to_owned())
    );
}
