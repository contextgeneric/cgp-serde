use cgp::prelude::*;
use cgp_serde::components::{
    CanDeserializeValue, CanSerializeValue, ValueDeserializer, ValueDeserializerComponent,
    ValueSerializer, ValueSerializerComponent,
};
use chrono::{DateTime, Utc};
use serde::de::Error;

pub struct SerializeTimestamp;

#[cgp_impl(SerializeTimestamp)]
#[uses(CanSerializeValue<i64>)]
impl ValueSerializer<DateTime<Utc>> {
    fn serialize<S>(&self, value: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.serialize(&value.timestamp(), serializer)
    }
}

#[cgp_impl(SerializeTimestamp)]
#[uses(CanDeserializeValue<'de, i64>)]
impl<'de> ValueDeserializer<'de, DateTime<Utc>> {
    fn deserialize<D>(&self, deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let timestamp = self.deserialize(deserializer)?;
        let date = DateTime::from_timestamp(timestamp, 0)
            .ok_or_else(|| Error::custom("invalid timestamp"))?;
        Ok(date)
    }
}
