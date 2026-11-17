use core::fmt;

use cgp::prelude::*;
use serde::de::Visitor;

use crate::components::{
    ValueDeserializer, ValueDeserializerComponent, ValueSerializer, ValueSerializerComponent,
};

/// Serializes any value as Serde's unit, and deserializes a unit into the type's `Default`.
///
/// It serves a type that carries no data, such as `Nil`, the payload the CGP variant derives give
/// a variant with no fields. With JSON, the unit is written as `null`.
pub struct SerializeUnit;

#[cgp_impl(SerializeUnit)]
impl<Value> ValueSerializer<Value> {
    fn serialize<S>(&self, _value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_unit()
    }
}

#[cgp_impl(SerializeUnit)]
impl<'de, Value> ValueDeserializer<'de, Value>
where
    Value: Default,
{
    fn deserialize<D>(&self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_unit(UnitVisitor(PhantomData))
    }
}

struct UnitVisitor<Value>(PhantomData<Value>);

impl<'de, Value> Visitor<'de> for UnitVisitor<Value>
where
    Value: Default,
{
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("unit")
    }

    fn visit_unit<E>(self) -> Result<Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::default())
    }
}
