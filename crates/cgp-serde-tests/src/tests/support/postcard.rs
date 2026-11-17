use cgp_serde::components::{CanDeserializeValue, CanSerializeValue};
use cgp_serde::types::{DeserializeWithContext, SerializeWithContext};
use serde::de::DeserializeSeed;

/// Serializes `value` through `context` with postcard.
pub fn to_postcard<Context, Value>(
    context: &Context,
    value: &Value,
) -> Result<Vec<u8>, postcard::Error>
where
    Context: CanSerializeValue<Value>,
{
    postcard::to_allocvec(&SerializeWithContext::new(context, value))
}

/// Deserializes postcard `bytes` through `context`, rejecting trailing bytes.
pub fn from_postcard<'de, Context, Value>(
    context: &Context,
    bytes: &'de [u8],
) -> Result<Value, postcard::Error>
where
    Context: CanDeserializeValue<'de, Value>,
{
    let mut deserializer = postcard::Deserializer::from_bytes(bytes);
    let value = DeserializeWithContext::new(context).deserialize(&mut deserializer)?;
    if deserializer.finalize()?.is_empty() {
        Ok(value)
    } else {
        Err(postcard::Error::DeserializeBadEncoding)
    }
}
