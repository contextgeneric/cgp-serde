use cgp::prelude::*;

/// Deserializes a value of type `Value` with the encoding the context chooses. See
/// [the component design page](https://contextgeneric.dev/docs/projects/cgp-serde/architecture/component-design).
#[cgp_component(ValueDeserializer)]
#[derive_delegate(UseDelegate<Value>)]
pub trait CanDeserializeValue<'de, Value> {
    fn deserialize<D>(&self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>;
}
