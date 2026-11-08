use core::marker::PhantomData;

use serde::de::DeserializeSeed;

use crate::components::CanDeserializeValue;

/// Wraps the context as a Serde seed, so a deserializer reads a value through its wiring. See
/// [the reference page](https://contextgeneric.dev/docs/projects/cgp-serde/reference/types/deserialize_with_context).
pub struct DeserializeWithContext<'a, Context, Value> {
    pub context: &'a Context,
    pub phantom: PhantomData<Value>,
}

impl<'a, Context, Value> DeserializeWithContext<'a, Context, Value> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context,
            phantom: PhantomData,
        }
    }
}

impl<'de, 'a, Context, Value> DeserializeSeed<'de> for DeserializeWithContext<'a, Context, Value>
where
    Context: CanDeserializeValue<'de, Value>,
{
    type Value = Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        self.context.deserialize(deserializer)
    }
}
