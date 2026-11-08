use cgp::prelude::*;

use crate::components::{
    CanDeserializeValue, CanSerializeValue, ValueDeserializer, ValueDeserializerComponent,
    ValueSerializer, ValueSerializerComponent,
};

/// Encodes a value by converting it into `Target` with `Into`, and decodes one by converting from
/// `Target` instead. See
/// [the reference page](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/serialize_from).
pub struct SerializeFrom<Target>(pub PhantomData<Target>);

#[cgp_impl(SerializeFrom<Target>)]
#[uses(CanSerializeValue<Target>)]
impl<Value, Target> ValueSerializer<Value>
where
    Value: Clone + Into<Target>,
{
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let target = value.clone().into();
        self.serialize(&target, serializer)
    }
}

#[cgp_impl(SerializeFrom<Source>)]
#[uses(CanDeserializeValue<'a, Source>)]
impl<'a, Value, Source> ValueDeserializer<'a, Value>
where
    Source: Into<Value>,
{
    fn deserialize<D>(&self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let target = self.deserialize(deserializer)?;
        Ok(target.into())
    }
}
