use alloc::string::String;

use cgp::prelude::*;
use cgp_serde::components::{
    CanDeserializeValue, CanSerializeValue, ValueDeserializer, ValueDeserializerComponent,
    ValueSerializer, ValueSerializerComponent,
};
use chrono::{DateTime, Utc};
use serde::de::Error;

pub struct SerializeRfc3339Date;

#[cgp_impl(SerializeRfc3339Date)]
#[uses(CanSerializeValue<String>)]
impl ValueSerializer<DateTime<Utc>> {
    fn serialize<S>(&self, value: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.serialize(&value.to_rfc3339(), serializer)
    }
}

#[cgp_impl(SerializeRfc3339Date)]
#[uses(CanDeserializeValue<'de, String>)]
impl<'de> ValueDeserializer<'de, DateTime<Utc>> {
    fn deserialize<D>(&self, deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let date_string = self.deserialize(deserializer)?;
        let date = DateTime::parse_from_rfc3339(&date_string).map_err(Error::custom)?;
        Ok(date.into())
    }
}
