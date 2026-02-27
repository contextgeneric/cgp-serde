use core::ops::Deref;

use cgp::prelude::*;

use crate::components::{CanSerializeValue, ValueSerializer, ValueSerializerComponent};

#[cgp_impl(new SerializeDeref)]
#[uses(CanSerializeValue<Value::Target>)]
impl<Value> ValueSerializer<Value>
where
    Value: Deref,
{
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.serialize(value.deref(), serializer)
    }
}
