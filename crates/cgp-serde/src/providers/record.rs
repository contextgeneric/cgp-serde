use alloc::string::String;

use cgp::core::field::traits::StaticString;
use cgp::extra::field::impls::{FinalizeOptional, HasOptionalBuilder, SetOptional};
use cgp::prelude::*;
use serde::de::{Error, IgnoredAny, MapAccess, Visitor};

use crate::components::{CanDeserializeValue, ValueDeserializer, ValueDeserializerComponent};
use crate::types::DeserializeWithContext;

pub struct DeserializeRecordFields;

#[cgp_impl(DeserializeRecordFields)]
impl<'de, Record, Builder> ValueDeserializer<'de, Record>
where
    Record: HasOptionalBuilder<Builder = Builder> + HasFields,
    Record::Fields: HandleMapEntry<'de, Self, Builder>,
    Builder: FinalizeOptional<Target = Record>,
{
    fn deserialize<D>(&self, deserializer: D) -> Result<Record, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(MapVisitor {
            context: self,
            phantom: PhantomData,
        })
    }
}

struct MapVisitor<'a, Context, Record> {
    context: &'a Context,
    phantom: PhantomData<Record>,
}

impl<'de, 'a, Context, Record, Builder> Visitor<'de> for MapVisitor<'a, Context, Record>
where
    Record: HasOptionalBuilder<Builder = Builder> + HasFields,
    Record::Fields: HandleMapEntry<'de, Context, Builder>,
    Builder: FinalizeOptional<Target = Record>,
{
    type Value = Record;

    fn expecting(&self, formatter: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
        write!(formatter, "map")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut builder = Record::optional_builder();

        while let Some(key) = map.next_key::<String>()? {
            builder = Record::Fields::handle_map_entry(&mut map, &key, self.context, builder)?;
        }

        let value = builder
            .finalize_optional()
            .map_err(|field| M::Error::custom(format_args!("missing field: {field}")))?;

        Ok(value)
    }
}

trait HandleMapEntry<'de, Context, Builder> {
    fn handle_map_entry<M: MapAccess<'de>>(
        map: &mut M,
        key: &str,
        context: &Context,
        builder: Builder,
    ) -> Result<Builder, M::Error>;
}

impl<'de, Context, Builder, Tag, Value, Tail> HandleMapEntry<'de, Context, Builder>
    for Cons<Field<Tag, Value>, Tail>
where
    Tag: StaticString,
    Tail: HandleMapEntry<'de, Context, Builder>,
    Context: CanDeserializeValue<'de, Value>,
    Builder: SetOptional<Tag, Value = Value>,
{
    fn handle_map_entry<M: MapAccess<'de>>(
        map: &mut M,
        key: &str,
        context: &Context,
        builder: Builder,
    ) -> Result<Builder, M::Error> {
        if key == Tag::VALUE {
            let value = map.next_value_seed(DeserializeWithContext {
                context,
                phantom: PhantomData::<Value>,
            })?;

            let (replaced, builder) = builder.set_optional(PhantomData, value);

            if replaced.is_some() {
                Err(M::Error::custom(format_args!(
                    "duplicate field: {}",
                    Tag::VALUE,
                )))
            } else {
                Ok(builder)
            }
        } else {
            Tail::handle_map_entry(map, key, context, builder)
        }
    }
}

impl<'de, Context, Builder> HandleMapEntry<'de, Context, Builder> for Nil {
    fn handle_map_entry<M: MapAccess<'de>>(
        map: &mut M,
        _key: &str,
        _context: &Context,
        builder: Builder,
    ) -> Result<Builder, M::Error> {
        map.next_value::<IgnoredAny>()?;

        Ok(builder)
    }
}
