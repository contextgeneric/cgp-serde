use alloc::string::String;
use alloc::vec::Vec;

use base64::prelude::*;
use cgp::prelude::*;
use cgp_serde::components::{
    CanDeserializeValue, CanSerializeValue, ValueDeserializer, ValueDeserializerComponent,
    ValueSerializer, ValueSerializerComponent,
};
use serde::de::Error;

pub struct SerializeBase64;

#[cgp_impl(SerializeBase64)]
impl<Context, Value> ValueSerializer<Value> for Context
where
    Value: AsRef<[u8]>,
    Context: CanSerializeValue<String>,
{
    fn serialize<S>(context: &Context, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let str_value = BASE64_STANDARD.encode(value);
        context.serialize(&str_value, serializer)
    }
}

#[cgp_impl(SerializeBase64)]
impl<'de, Context> ValueDeserializer<'de, Vec<u8>> for Context
where
    Context: CanDeserializeValue<'de, String>,
{
    fn deserialize<D>(context: &Context, deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let str_value = context.deserialize(deserializer)?;
        BASE64_STANDARD.decode(str_value).map_err(Error::custom)
    }
}
