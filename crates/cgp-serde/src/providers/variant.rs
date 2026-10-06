use core::fmt::{self, Display};

use cgp::core::field::traits::StaticString;
use cgp::prelude::*;
use serde::de::{DeserializeSeed, EnumAccess, Error, VariantAccess, Visitor};

use crate::components::{CanDeserializeValue, ValueDeserializer, ValueDeserializerComponent};
use crate::types::DeserializeWithContext;

/// Deserializes an enum from Serde's externally tagged form, `{"Variant": payload}`, deserializing
/// the payload through the context.
///
/// The enum needs `#[derive(HasFields)]`, which `#[derive(CgpVariant)]` and `#[derive(CgpData)]`
/// include, and every variant must hold one unnamed payload or no fields, as those derives require.
/// A variant with no fields has the payload `Nil`, and the context's wiring for `Nil` decides its
/// format. A variant is found by its name in text formats and by its declaration index in binary
/// ones.
pub struct DeserializeVariantFields;

#[cgp_impl(DeserializeVariantFields)]
impl<'de, Value> ValueDeserializer<'de, Value>
where
    Value: FromFields,
    Value::Fields: VariantsDeserializer<'de, Self>,
{
    fn deserialize<D>(&self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let fields = deserializer.deserialize_enum(
            enum_name::<Value>(),
            &[],
            EnumVisitor {
                context: self,
                phantom: PhantomData::<Value::Fields>,
            },
        )?;

        Ok(Value::from_fields(fields))
    }
}

/// The enum's own name, for formats that validate it or show it in errors, such as RON.
///
/// `HasFields` carries no type name, so this takes the last path segment of
/// [`core::any::type_name`] without its generic arguments. That formatting is not guaranteed
/// stable, but the name only labels the enum and never decides which variant is read.
pub(crate) fn enum_name<Value: ?Sized>() -> &'static str {
    let path = core::any::type_name::<Value>();
    let without_generics = path.split('<').next().unwrap_or(path);

    without_generics
        .rsplit("::")
        .next()
        .unwrap_or(without_generics)
}

struct EnumVisitor<'a, Context, Fields> {
    context: &'a Context,
    phantom: PhantomData<Fields>,
}

impl<'de, 'a, Context, Fields> Visitor<'de> for EnumVisitor<'a, Context, Fields>
where
    Fields: VariantsDeserializer<'de, Context>,
{
    type Value = Fields;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "enum")
    }

    fn visit_enum<A>(self, data: A) -> Result<Fields, A::Error>
    where
        A: EnumAccess<'de>,
    {
        let (index, variant) = data.variant_seed(VariantIndex::<Fields>(PhantomData))?;

        Fields::deserialize_variant(index, variant, self.context)
    }
}

/// Reads a variant identifier, by name or by index, as the variant's position in the list.
struct VariantIndex<Fields>(PhantomData<Fields>);

impl<'de, Fields> DeserializeSeed<'de> for VariantIndex<Fields>
where
    Fields: VariantNames,
{
    type Value = usize;

    fn deserialize<D>(self, deserializer: D) -> Result<usize, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_identifier(self)
    }
}

impl<'de, Fields> Visitor<'de> for VariantIndex<Fields>
where
    Fields: VariantNames,
{
    type Value = usize;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "variant identifier")
    }

    fn visit_u64<E: Error>(self, index: u64) -> Result<usize, E> {
        match usize::try_from(index) {
            Ok(index) if index < Fields::COUNT => Ok(index),
            _ => Err(E::custom(format_args!(
                "variant index {index} out of range, expected one of {}",
                ExpectedVariants::<Fields>(PhantomData),
            ))),
        }
    }

    fn visit_str<E: Error>(self, name: &str) -> Result<usize, E> {
        Fields::index_of(name.as_bytes(), 0).ok_or_else(|| {
            E::custom(format_args!(
                "unknown variant `{name}`, expected one of {}",
                ExpectedVariants::<Fields>(PhantomData),
            ))
        })
    }

    fn visit_bytes<E: Error>(self, name: &[u8]) -> Result<usize, E> {
        Fields::index_of(name, 0).ok_or_else(|| {
            E::custom(format_args!(
                "unknown variant `{}`, expected one of {}",
                name.escape_ascii(),
                ExpectedVariants::<Fields>(PhantomData),
            ))
        })
    }
}

/// Formats the variant names as `` `A`, `B` `` for an error message.
struct ExpectedVariants<Fields>(PhantomData<Fields>);

impl<Fields> Display for ExpectedVariants<Fields>
where
    Fields: VariantNames,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Fields::write_names(formatter, true)
    }
}

trait VariantNames {
    const COUNT: usize;

    fn index_of(name: &[u8], offset: usize) -> Option<usize>;

    fn write_names(formatter: &mut fmt::Formatter, first: bool) -> fmt::Result;
}

impl<Tag, Payload, Rest> VariantNames for Either<Field<Tag, Payload>, Rest>
where
    Tag: StaticString,
    Rest: VariantNames,
{
    const COUNT: usize = 1 + Rest::COUNT;

    fn index_of(name: &[u8], offset: usize) -> Option<usize> {
        if name == Tag::VALUE.as_bytes() {
            Some(offset)
        } else {
            Rest::index_of(name, offset + 1)
        }
    }

    fn write_names(formatter: &mut fmt::Formatter, first: bool) -> fmt::Result {
        if !first {
            write!(formatter, ", ")?;
        }

        write!(formatter, "`{}`", Tag::VALUE)?;
        Rest::write_names(formatter, false)
    }
}

impl VariantNames for Void {
    const COUNT: usize = 0;

    fn index_of(_name: &[u8], _offset: usize) -> Option<usize> {
        None
    }

    fn write_names(_formatter: &mut fmt::Formatter, _first: bool) -> fmt::Result {
        Ok(())
    }
}

trait VariantsDeserializer<'de, Context>: VariantNames + Sized {
    fn deserialize_variant<A>(
        index: usize,
        variant: A,
        context: &Context,
    ) -> Result<Self, A::Error>
    where
        A: VariantAccess<'de>;
}

impl<'de, Context, Tag, Payload, Rest> VariantsDeserializer<'de, Context>
    for Either<Field<Tag, Payload>, Rest>
where
    Tag: StaticString,
    Context: CanDeserializeValue<'de, Payload>,
    Rest: VariantsDeserializer<'de, Context>,
{
    fn deserialize_variant<A>(index: usize, variant: A, context: &Context) -> Result<Self, A::Error>
    where
        A: VariantAccess<'de>,
    {
        if index == 0 {
            let payload = variant.newtype_variant_seed(DeserializeWithContext {
                context,
                phantom: PhantomData::<Payload>,
            })?;

            Ok(Either::Left(payload.into()))
        } else {
            Rest::deserialize_variant(index - 1, variant, context).map(Either::Right)
        }
    }
}

impl<'de, Context> VariantsDeserializer<'de, Context> for Void {
    fn deserialize_variant<A>(
        index: usize,
        _variant: A,
        _context: &Context,
    ) -> Result<Self, A::Error>
    where
        A: VariantAccess<'de>,
    {
        // The identifier seed only returns indices below `COUNT`, so this is never reached.
        Err(A::Error::custom(format_args!(
            "variant index {index} out of range"
        )))
    }
}
