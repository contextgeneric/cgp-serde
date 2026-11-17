use core::fmt::Debug;

use cgp_serde::components::{CanDeserializeValue, CanSerializeValue};
use cgp_serde::types::{DeserializeWithContext, SerializeWithContext};
use serde::de::DeserializeSeed;

/// Serializes `value` through `context` as compact JSON.
pub fn to_json<Context, Value>(context: &Context, value: &Value) -> String
where
    Context: CanSerializeValue<Value>,
{
    serde_json::to_string(&SerializeWithContext::new(context, value)).unwrap()
}

/// Deserializes `input` through `context`, rejecting trailing input. The error is the
/// `serde_json` message, position included.
pub fn from_json<'de, Context, Value>(context: &Context, input: &'de str) -> Result<Value, String>
where
    Context: CanDeserializeValue<'de, Value>,
{
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let value = DeserializeWithContext::new(context)
        .deserialize(&mut deserializer)
        .map_err(|e| e.to_string())?;
    deserializer.end().map_err(|e| e.to_string())?;
    Ok(value)
}

/// Like [`from_json`], but reads through an `io::Read`, which cannot lend borrowed data.
pub fn from_json_reader<Context, Value>(context: &Context, input: &str) -> Result<Value, String>
where
    Context: for<'de> CanDeserializeValue<'de, Value>,
{
    let mut deserializer = serde_json::Deserializer::from_reader(input.as_bytes());
    let value = DeserializeWithContext::new(context)
        .deserialize(&mut deserializer)
        .map_err(|e| e.to_string())?;
    deserializer.end().map_err(|e| e.to_string())?;
    Ok(value)
}

/// Asserts that `value` serializes to exactly `expected` and that `expected` deserializes back
/// to `value`.
pub fn assert_json_round_trip<Context, Value>(context: &Context, value: Value, expected: &str)
where
    Context: CanSerializeValue<Value> + for<'de> CanDeserializeValue<'de, Value>,
    Value: Debug + PartialEq,
{
    assert_eq!(to_json(context, &value), expected);
    assert_eq!(
        from_json::<Context, Value>(context, expected).unwrap(),
        value
    );
}
