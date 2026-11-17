use cgp_serde::components::{CanDeserializeValue, CanSerializeValue};
use cgp_serde::types::{DeserializeWithContext, SerializeWithContext};
use serde::de::DeserializeSeed;

/// Serializes `value` through `context` as compact RON.
pub fn to_ron<Context, Value>(context: &Context, value: &Value) -> String
where
    Context: CanSerializeValue<Value>,
{
    ron::to_string(&SerializeWithContext::new(context, value)).unwrap()
}

/// Deserializes RON `input` through `context`, rejecting trailing input.
pub fn from_ron<'de, Context, Value>(context: &Context, input: &'de str) -> Result<Value, String>
where
    Context: CanDeserializeValue<'de, Value>,
{
    let mut deserializer = ron::Deserializer::from_str(input).map_err(|e| e.to_string())?;
    let value = DeserializeWithContext::new(context)
        .deserialize(&mut deserializer)
        .map_err(|e| e.to_string())?;
    deserializer.end().map_err(|e| e.to_string())?;
    Ok(value)
}
