use cgp::prelude::*;
use cgp_serde::components::{CanDeserializeValue, ValueDeserializer, ValueDeserializerComponent};

use crate::traits::CanAlloc;

#[cgp_impl(new DeserializeAndAllocate)]
impl<'de, 'a, Context, Value> ValueDeserializer<'de, &'a Value> for Context
where
    Context: CanAlloc<'a, Value> + CanDeserializeValue<'de, Value>,
{
    fn deserialize<D>(&self, deserializer: D) -> Result<&'a Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = self.deserialize(deserializer)?;
        let value = self.alloc(value);

        Ok(value)
    }
}
