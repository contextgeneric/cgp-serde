use cgp::prelude::*;
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

use crate::components::{
    ValueDeserializer, ValueDeserializerComponent, ValueSerializer, ValueSerializerComponent,
};

pub struct UseSerde;

#[cgp_impl(UseSerde)]
impl<Value> ValueSerializer<Value>
where
    Value: SerdeSerialize,
{
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        value.serialize(serializer)
    }
}

#[cgp_impl(UseSerde)]
impl<'a, Value> ValueDeserializer<'a, Value>
where
    Value: SerdeDeserialize<'a>,
{
    fn deserialize<D>(&self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        Value::deserialize(deserializer)
    }
}
