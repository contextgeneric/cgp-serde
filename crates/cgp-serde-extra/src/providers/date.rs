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
impl<Context> ValueSerializer<DateTime<Utc>> for Context
where
    Context: CanSerializeValue<String>,
{
    fn serialize<S>(
        context: &Context,
        value: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        context.serialize(&value.to_rfc3339(), serializer)
    }
}

#[cgp_impl(SerializeRfc3339Date)]
impl<'de, Context> ValueDeserializer<'de, DateTime<Utc>> for Context
where
    Context: CanDeserializeValue<'de, String>,
{
    fn deserialize<D>(context: &Context, deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let date_string = context.deserialize(deserializer)?;
        let date = DateTime::parse_from_rfc3339(&date_string).map_err(Error::custom)?;
        Ok(date.into())
    }
}
