use core::ops::Deref;

use cgp::prelude::*;

use crate::components::{CanSerializeValue, ValueSerializer, ValueSerializerComponent};

/// Serializes a smart pointer or reference as the value it points to. See
/// [the reference page](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/serialize_deref).
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
