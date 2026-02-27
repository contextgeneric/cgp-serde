use cgp::prelude::*;
use serde::de::Visitor;

use crate::components::{ValueDeserializer, ValueDeserializerComponent};

#[cgp_impl(new DeserializeDefault<Provider>)]
#[use_provider(Provider: ValueDeserializer<'a, Value>)]
impl<'a, Value, Provider> ValueDeserializer<'a, Value>
where
    Value: Default,
{
    fn deserialize<D>(&self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        deserializer.deserialize_option(DefaultVisitor {
            context: self,
            phantom: PhantomData::<(Value, Provider)>,
        })
    }
}

struct DefaultVisitor<'a, Context, Value, Provider> {
    pub context: &'a Context,
    pub phantom: PhantomData<(Value, Provider)>,
}

impl<'a, 'de, Context, Value, Provider> Visitor<'de>
    for DefaultVisitor<'a, Context, Value, Provider>
where
    Value: Default,
    Provider: ValueDeserializer<'de, Context, Value>,
{
    type Value = Value;

    fn expecting(&self, formatter: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
        formatter.write_str("optional")
    }

    fn visit_none<E>(self) -> Result<Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::default())
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Provider::deserialize(self.context, deserializer)
    }
}
