use cgp::prelude::*;

use crate::code::DeserializeJson;
use crate::providers::DeserializeFromJsonString;

pub trait CanDeserializeJsonString<T>: HasErrorType {
    fn deserialize_json_string(&self, serialized: &str) -> Result<T, Self::Error>;
}

impl<Context, T> CanDeserializeJsonString<T> for Context
where
    Context: HasErrorType,
    DeserializeFromJsonString:
        for<'a> TryComputer<Context, DeserializeJson<T>, &'a str, Output = T>,
{
    fn deserialize_json_string(&self, serialized: &str) -> Result<T, Self::Error> {
        <DeserializeFromJsonString>::try_compute(self, PhantomData, serialized)
    }
}
