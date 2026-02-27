use core::fmt::Display;

use cgp::prelude::*;
use serde::de::Error as _;
use serde::ser::Error as _;

use crate::components::{
    CanDeserializeValue, CanSerializeValue, ValueDeserializer, ValueDeserializerComponent,
    ValueSerializer, ValueSerializerComponent,
};

pub struct TrySerializeFrom<Target>(pub PhantomData<Target>);

#[cgp_impl(TrySerializeFrom<Target>)]
#[uses(CanSerializeValue<Target>)]
impl<Value, Target> ValueSerializer<Value>
where
    Value: Clone + TryInto<Target, Error: Display>,
{
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let target = value.clone().try_into().map_err(S::Error::custom)?;
        self.serialize(&target, serializer)
    }
}

#[cgp_impl(TrySerializeFrom<Source>)]
#[uses(CanDeserializeValue<'a, Source>)]
impl<'a, Value, Source> ValueDeserializer<'a, Value>
where
    Source: TryInto<Value, Error: Display>,
{
    fn deserialize<D>(&self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let target = self.deserialize(deserializer)?;
        target.try_into().map_err(D::Error::custom)
    }
}
