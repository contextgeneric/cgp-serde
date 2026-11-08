use alloc::string::String;

use cgp::prelude::*;
use cgp_serde::components::CanSerializeValue;
use cgp_serde::types::SerializeWithContext;

/// Serializes a value to a compact JSON string through the context. See
/// [the reference page](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/serialize_to_json_string).
#[cgp_impl(new SerializeToJsonString)]
#[use_type(HasErrorType.Error)]
#[uses(CanSerializeValue<Value>, CanRaiseError<serde_json::Error>)]
impl<Code, Value> TryComputer<Code, &Value> {
    type Output = String;

    fn try_compute(&self, _code: PhantomData<Code>, value: &Value) -> Result<String, Error> {
        serde_json::to_string(&SerializeWithContext::new(self, value)).map_err(Self::raise_error)
    }
}
