use cgp::prelude::*;
use cgp_serde::components::{CanDeserializeValue, ValueDeserializer, ValueDeserializerComponent};

use crate::traits::CanAlloc;

/// Deserializes an owned value and allocates it, returning a reference that outlives the input. See
/// [the reference page](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/deserialize_and_allocate).
#[cgp_impl(new DeserializeAndAllocate)]
#[uses(CanAlloc<'a, Value>, CanDeserializeValue<'de, Value>)]
impl<'de, 'a, Value> ValueDeserializer<'de, &'a Value> {
    fn deserialize<D>(&self, deserializer: D) -> Result<&'a Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = self.deserialize(deserializer)?;
        let value = self.alloc(value);

        Ok(value)
    }
}
