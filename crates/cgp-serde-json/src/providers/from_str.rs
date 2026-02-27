use cgp::prelude::*;
use serde_json::de::StrRead;

use crate::providers::DeserializeFromJsonReader;

pub struct DeserializeFromJsonString<InDeserializer = DeserializeFromJsonReader>(
    pub PhantomData<InDeserializer>,
);

#[cgp_impl(DeserializeFromJsonString<InDeserializer>)]
#[use_type(HasErrorType::Error)]
impl<Code, Value, S, InDeserializer> TryComputer<Code, S>
where
    InDeserializer: for<'a> TryComputer<Self, Code, StrRead<'a>, Output = Value>,
    S: AsRef<str>,
{
    type Output = Value;

    fn try_compute(&self, code: PhantomData<Code>, source: S) -> Result<Value, Error> {
        InDeserializer::try_compute(self, code, StrRead::new(source.as_ref()))
    }
}
