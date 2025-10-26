use alloc::string::String;

use cgp::prelude::*;
use cgp_serde::components::CanSerializeValue;
use cgp_serde::types::SerializeWithContext;
use serde_json::Error;

#[cgp_impl(new SerializeToJsonString)]
impl<Context, Code, Value> TryComputer<Code, &Value> for Context
where
    Context: CanSerializeValue<Value> + CanRaiseError<Error>,
{
    type Output = String;

    fn try_compute(
        context: &Context,
        _code: PhantomData<Code>,
        value: &Value,
    ) -> Result<String, Context::Error> {
        serde_json::to_string(&SerializeWithContext::new(context, value))
            .map_err(Context::raise_error)
    }
}
