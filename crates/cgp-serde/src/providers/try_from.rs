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
impl<Context, Value, Target> ValueSerializer<Value> for Context
where
    Value: Clone + TryInto<Target, Error: Display>,
    Context: CanSerializeValue<Target>,
{
    fn serialize<S>(context: &Context, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let target = value.clone().try_into().map_err(S::Error::custom)?;
        context.serialize(&target, serializer)
    }
}

#[cgp_impl(TrySerializeFrom<Source>)]
impl<'a, Context, Value, Source> ValueDeserializer<'a, Value> for Context
where
    Context: CanDeserializeValue<'a, Source>,
    Source: TryInto<Value, Error: Display>,
{
    fn deserialize<D>(context: &Context, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let target = context.deserialize(deserializer)?;
        target.try_into().map_err(D::Error::custom)
    }
}
