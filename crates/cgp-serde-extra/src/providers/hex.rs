use alloc::string::String;
use core::fmt::Display;

use cgp::prelude::*;
use cgp_serde::components::{
    CanDeserializeValue, CanSerializeValue, ValueDeserializer, ValueDeserializerComponent,
    ValueSerializer, ValueSerializerComponent,
};
use hex::{FromHex, ToHex};
use serde::de::Error;

pub struct SerializeHex;

#[cgp_impl(SerializeHex)]
impl<Context, Value> ValueSerializer<Value> for Context
where
    Value: ToHex,
    Context: CanSerializeValue<String>,
{
    fn serialize<S>(context: &Context, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let str_value = value.encode_hex::<String>();
        context.serialize(&str_value, serializer)
    }
}

#[cgp_impl(SerializeHex)]
impl<'de, Context, Value> ValueDeserializer<'de, Value> for Context
where
    Context: CanDeserializeValue<'de, String>,
    Value: FromHex<Error: Display>,
{
    fn deserialize<D>(context: &Context, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let str_value = context.deserialize(deserializer)?;
        Value::from_hex(str_value).map_err(Error::custom)
    }
}
