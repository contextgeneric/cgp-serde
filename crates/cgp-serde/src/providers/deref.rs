use core::ops::Deref;

use cgp::prelude::*;

use crate::components::{CanSerializeValue, ValueSerializer, ValueSerializerComponent};

#[cgp_impl(new SerializeDeref)]
impl<Context, Value> ValueSerializer<Value> for Context
where
    Value: Deref,
    Context: CanSerializeValue<Value::Target>,
{
    fn serialize<S>(context: &Context, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        context.serialize(value.deref(), serializer)
    }
}
