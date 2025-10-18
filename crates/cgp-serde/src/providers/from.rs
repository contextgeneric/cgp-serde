use cgp::prelude::*;

use crate::components::{
    CanDeserializeValue, CanSerializeValue, ValueDeserializer, ValueDeserializerComponent,
    ValueSerializer, ValueSerializerComponent,
};

pub struct SerializeFrom<Target>(pub PhantomData<Target>);

#[cgp_impl(SerializeFrom<Target>)]
impl<Context, Value, Target> ValueSerializer<Value> for Context
where
    Value: Clone + Into<Target>,
    Context: CanSerializeValue<Target>,
{
    fn serialize<S>(context: &Context, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let target = value.clone().into();
        context.serialize(&target, serializer)
    }
}

#[cgp_impl(SerializeFrom<Source>)]
impl<'a, Context, Value, Source> ValueDeserializer<'a, Value> for Context
where
    Context: CanDeserializeValue<'a, Source>,
    Source: Into<Value>,
{
    fn deserialize<D>(context: &Context, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let target = context.deserialize(deserializer)?;
        Ok(target.into())
    }
}
