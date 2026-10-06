use cgp::core::field::traits::StaticString;
use cgp::prelude::*;

use super::variant::enum_name;
use crate::components::{CanSerializeValue, ValueSerializer, ValueSerializerComponent};
use crate::types::SerializeWithContext;

/// Serializes an enum in Serde's externally tagged form, `{"Variant": payload}`, serializing the
/// payload through the context.
///
/// The enum needs `#[derive(HasFields)]`, which `#[derive(CgpVariant)]` and `#[derive(CgpData)]`
/// include, and every variant must hold one unnamed payload or no fields, as those derives require.
/// A variant with no fields has the payload `Nil`, which the context must wire; with `Nil` wired to
/// `SerializeUnit`, it is written as `{"Variant": null}`, unlike the forms Serde's derive writes for
/// such a variant. The variant index passed to the format is its declaration position, as with
/// Serde's derive.
///
/// The enum must be `'static`: the bound on its borrowed field view must hold for every lifetime,
/// which Rust can only prove for a `'static` type, so an enum such as `Token<'a>` cannot use it.
pub struct SerializeVariantFields;

#[cgp_impl(SerializeVariantFields)]
impl<Value> ValueSerializer<Value>
where
    Value: ToFieldsRef,
    for<'a> Value::FieldsRef<'a>: VariantsSerializer<Self>,
{
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        value
            .to_fields_ref()
            .serialize_variant(self, serializer, enum_name::<Value>(), 0)
    }
}

trait VariantsSerializer<Context> {
    fn serialize_variant<S>(
        self,
        context: &Context,
        serializer: S,
        name: &'static str,
        index: u32,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;
}

impl<'a, Context, Tag, Payload, Rest> VariantsSerializer<Context>
    for Either<Field<Tag, &'a Payload>, Rest>
where
    Tag: StaticString,
    Context: CanSerializeValue<Payload>,
    Rest: VariantsSerializer<Context>,
{
    fn serialize_variant<S>(
        self,
        context: &Context,
        serializer: S,
        name: &'static str,
        index: u32,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Either::Left(field) => serializer.serialize_newtype_variant(
                name,
                index,
                Tag::VALUE,
                &SerializeWithContext {
                    context,
                    value: field.value,
                },
            ),
            Either::Right(rest) => rest.serialize_variant(context, serializer, name, index + 1),
        }
    }
}

/// A variant with no fields, whose borrowed payload is `Nil` itself rather than a reference to one,
/// since the borrowed view of an empty product has nothing to borrow. Its payload is serialized
/// through the context as `&Nil`, so the context's wiring for `Nil` decides its format.
impl<Context, Tag, Rest> VariantsSerializer<Context> for Either<Field<Tag, Nil>, Rest>
where
    Tag: StaticString,
    Context: CanSerializeValue<Nil>,
    Rest: VariantsSerializer<Context>,
{
    fn serialize_variant<S>(
        self,
        context: &Context,
        serializer: S,
        name: &'static str,
        index: u32,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Either::Left(_) => serializer.serialize_newtype_variant(
                name,
                index,
                Tag::VALUE,
                &SerializeWithContext {
                    context,
                    value: &Nil,
                },
            ),
            Either::Right(rest) => rest.serialize_variant(context, serializer, name, index + 1),
        }
    }
}

impl<Context> VariantsSerializer<Context> for Void {
    fn serialize_variant<S>(
        self,
        _context: &Context,
        _serializer: S,
        _name: &'static str,
        _index: u32,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {}
    }
}
