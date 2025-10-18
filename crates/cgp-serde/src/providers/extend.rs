use cgp::prelude::*;
use serde::de::{DeserializeSeed, Visitor};

use crate::components::{CanDeserializeValue, ValueDeserializer, ValueDeserializerComponent};

pub struct DeserializeExtend;

#[cgp_impl(DeserializeExtend)]
impl<'de, Context, Value, Item> ValueDeserializer<'de, Value> for Context
where
    Value: Default + IntoIterator<Item = Item> + Extend<Item>,
    Context: CanDeserializeValue<'de, Item>,
{
    fn deserialize<D>(context: &Context, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(DeserializeExtendVisitor {
            context,
            phantom: PhantomData,
        })
    }
}

struct DeserializeExtendVisitor<'a, Context, Value, Item> {
    context: &'a Context,
    phantom: PhantomData<(Value, Item)>,
}

impl<'de, 'a, Context, Value, Item> Visitor<'de>
    for DeserializeExtendVisitor<'a, Context, Value, Item>
where
    Value: Default + Extend<Item>,
    Context: CanDeserializeValue<'de, Item>,
{
    type Value = Value;

    fn expecting(&self, formatter: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
        write!(formatter, "sequence")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut value = Value::default();

        while let Some(item) = seq.next_element_seed(DeserializeExtendSeed {
            context: self.context,
            phantom: PhantomData,
        })? {
            value.extend(core::iter::once(item));
        }

        Ok(value)
    }
}

struct DeserializeExtendSeed<'a, Context, Item> {
    context: &'a Context,
    phantom: PhantomData<Item>,
}

impl<'de, 'a, Context, Item> DeserializeSeed<'de> for DeserializeExtendSeed<'a, Context, Item>
where
    Context: CanDeserializeValue<'de, Item>,
{
    type Value = Item;

    fn deserialize<D>(self, deserializer: D) -> Result<Item, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        self.context.deserialize(deserializer)
    }
}
