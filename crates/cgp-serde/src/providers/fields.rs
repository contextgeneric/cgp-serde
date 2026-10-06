use cgp::core::field::traits::StaticString;
use cgp::prelude::*;
use serde::ser::SerializeMap;

use crate::components::{CanSerializeValue, ValueSerializer, ValueSerializerComponent};
use crate::types::SerializeWithContext;

/// Serializes a struct as a map from each field's name to its value, serializing every field value
/// through the context.
///
/// The struct needs `#[derive(HasField, HasFields)]`, which `#[derive(CgpData)]` includes. Fields are
/// written in declaration order under their Rust names, and the map is opened without a length, so
/// length-prefixed formats such as postcard reject the output.
pub struct SerializeRecordFields;

#[cgp_impl(SerializeRecordFields)]
impl<Value> ValueSerializer<Value>
where
    Value: HasFields,
    Value::Fields: FieldsSerializer<Self, Value>,
{
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = serializer.serialize_map(None)?;
        Value::Fields::serialize_fields(self, value, s)
    }
}

trait FieldsSerializer<Context, Value> {
    fn serialize_fields<S>(
        context: &Context,
        value: &Value,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: SerializeMap;
}

impl<Context, Value, Tag, FieldValue, Rest> FieldsSerializer<Context, Value>
    for Cons<Field<Tag, FieldValue>, Rest>
where
    Tag: StaticString,
    Value: HasField<Tag, Value = FieldValue>,
    Context: CanSerializeValue<FieldValue>,
    Rest: FieldsSerializer<Context, Value>,
{
    fn serialize_fields<S>(
        context: &Context,
        value: &Value,
        mut serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: SerializeMap,
    {
        let field_value = value.get_field(PhantomData);
        serializer.serialize_entry(
            Tag::VALUE,
            &SerializeWithContext {
                context,
                value: field_value,
            },
        )?;
        Rest::serialize_fields(context, value, serializer)
    }
}

impl<Context, Value> FieldsSerializer<Context, Value> for Nil {
    fn serialize_fields<S>(
        _context: &Context,
        _value: &Value,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: SerializeMap,
    {
        serializer.end()
    }
}
