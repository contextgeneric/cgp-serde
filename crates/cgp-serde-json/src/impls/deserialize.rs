use cgp::prelude::*;

use crate::code::DeserializeJson;
use crate::providers::DeserializeFromJsonString;

#[cgp_fn(CanDeserializeJsonString)]
#[use_type(HasErrorType::Error)]
pub fn deserialize_json_string<T>(&self, serialized: &str) -> Result<T, Error>
where
    DeserializeFromJsonString: for<'a> TryComputer<Self, DeserializeJson<T>, &'a str, Output = T>,
{
    <DeserializeFromJsonString>::try_compute(self, PhantomData, serialized)
}
