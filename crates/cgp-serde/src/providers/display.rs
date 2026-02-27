use alloc::string::{String, ToString};
use core::fmt::Display;
use core::str::FromStr;

use cgp::prelude::*;
use serde::Serializer;
use serde::de::Error;

use crate::components::{
    CanDeserializeValue, CanSerializeValue, ValueDeserializer, ValueDeserializerComponent,
    ValueSerializer, ValueSerializerComponent,
};

#[cgp_impl(new SerializeWithDisplay)]
#[uses(CanSerializeValue<String>)]
impl<Value> ValueSerializer<Value>
where
    Value: Display,
{
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let str_value = value.to_string();
        self.serialize(&str_value, serializer)
    }
}

#[cgp_impl(new DeserializeWithFromStr)]
#[uses(CanDeserializeValue<'a, &'a str>)]
impl<'a, Value> ValueDeserializer<'a, Value>
where
    Value: FromStr<Err: Display>,
{
    fn deserialize<D>(&self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let str_value = self.deserialize(deserializer)?;
        Value::from_str(str_value).map_err(D::Error::custom)
    }
}
