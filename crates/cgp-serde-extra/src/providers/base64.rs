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
#[uses(CanSerializeValue<String>)]
impl<Value> ValueSerializer<Value>
where
    Value: AsRef<[u8]>,
{
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let str_value = BASE64_STANDARD.encode(value);
        self.serialize(&str_value, serializer)
    }
}

#[cgp_impl(SerializeBase64)]
#[uses(CanDeserializeValue<'de, String>)]
impl<'de> ValueDeserializer<'de, Vec<u8>> {
    fn deserialize<D>(&self, deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let str_value = self.deserialize(deserializer)?;
        BASE64_STANDARD.decode(str_value).map_err(Error::custom)
    }
}
