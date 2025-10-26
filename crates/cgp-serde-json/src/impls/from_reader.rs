use cgp::prelude::*;
use cgp_serde::components::CanDeserializeValue;
use serde_json::de::Read;
use serde_json::{Deserializer, Error};

use crate::code::DeserializeJson;

#[cgp_impl(new DeserializeFromJsonReader)]
impl<Context, Value, R, 'de> TryComputer<DeserializeJson<Value>, R> for Context
where
    R: Read<'de>,
    Context: CanDeserializeValue<'de, Value> + CanRaiseError<Error>,
{
    type Output = Value;

    fn try_compute(
        context: &Context,
        _code: PhantomData<DeserializeJson<Value>>,
        source: R,
    ) -> Result<Value, Context::Error> {
        let mut deserializer = Deserializer::new(source);
        let value = context
            .deserialize(&mut deserializer)
            .map_err(Context::raise_error)?;
        deserializer.end().map_err(Context::raise_error)?;

        Ok(value)
    }
}
