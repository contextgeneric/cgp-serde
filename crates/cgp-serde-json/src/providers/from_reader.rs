use cgp::prelude::*;
use cgp_serde::components::CanDeserializeValue;
use serde_json::Deserializer;
use serde_json::de::Read;

use crate::code::DeserializeJson;

#[cgp_impl(new DeserializeFromJsonReader)]
#[use_type(HasErrorType::Error)]
#[uses(CanDeserializeValue<'de, Value>, CanRaiseError<serde_json::Error>)]
impl<Value, R, 'de> TryComputer<DeserializeJson<Value>, R>
where
    R: Read<'de>,
{
    type Output = Value;

    fn try_compute(
        &self,
        _code: PhantomData<DeserializeJson<Value>>,
        source: R,
    ) -> Result<Value, Error> {
        let mut deserializer = Deserializer::new(source);

        let value = self
            .deserialize(&mut deserializer)
            .map_err(Self::raise_error)?;

        deserializer.end().map_err(Self::raise_error)?;

        Ok(value)
    }
}
