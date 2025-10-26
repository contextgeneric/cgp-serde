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
impl<Context, Value> ValueSerializer<Value> for Context
where
    Context: CanSerializeValue<String>,
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
impl<'a, Context, Value> ValueDeserializer<'a, Value> for Context
where
    Context: CanDeserializeValue<'a, &'a str>,
    Value: FromStr<Err: Display>,
{
    fn deserialize<D>(context: &Context, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let str_value = context.deserialize(deserializer)?;
        Value::from_str(str_value).map_err(D::Error::custom)
    }
}
