use cgp::prelude::*;
use cgp_serde::components::{CanDeserializeValue, ValueDeserializer, ValueDeserializerComponent};

use crate::traits::CanAlloc;

#[cgp_new_provider]
impl<'de, 'a, Context, Value> ValueDeserializer<'de, Context, &'a Value> for DeserializeAndAllocate
where
    Context: CanAlloc<'a, Value> + CanDeserializeValue<'de, Value>,
{
    fn deserialize<D>(context: &Context, deserializer: D) -> Result<&'a Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = context.deserialize(deserializer)?;
        let value = context.alloc(value);

        Ok(value)
    }
}
