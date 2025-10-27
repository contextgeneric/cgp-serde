use cgp::prelude::*;
use serde_json::de::StrRead;

use crate::providers::DeserializeFromJsonReader;

pub struct DeserializeFromJsonString<InDeserializer = DeserializeFromJsonReader>(
    pub PhantomData<InDeserializer>,
);

#[cgp_impl(DeserializeFromJsonString<InDeserializer>)]
impl<Context, Code, Value, S, InDeserializer> TryComputer<Code, S> for Context
where
    Context: HasErrorType,
    InDeserializer: for<'a> TryComputer<Context, Code, StrRead<'a>, Output = Value>,
    S: AsRef<str>,
{
    type Output = Value;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        source: S,
    ) -> Result<Value, Context::Error> {
        InDeserializer::try_compute(context, code, StrRead::new(source.as_ref()))
    }
}
