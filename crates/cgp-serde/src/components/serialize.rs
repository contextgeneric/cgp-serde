use cgp::prelude::*;

/// Serializes a value of type `Value` with the encoding the context chooses. See
/// [the component design page](https://contextgeneric.dev/docs/projects/cgp-serde/architecture/component-design).
#[cgp_component(ValueSerializer)]
#[derive_delegate(UseDelegate<Value>)]
pub trait CanSerializeValue<Value: ?Sized> {
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;
}
