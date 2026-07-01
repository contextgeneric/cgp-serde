use cgp::prelude::*;

#[cgp_component(ValueDeserializer)]
#[derive_delegate(UseDelegate<Value>)]
pub trait CanDeserializeValue<'de, Value> {
    fn deserialize<D>(&self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>;
}
