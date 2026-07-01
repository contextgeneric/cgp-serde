use cgp::prelude::*;

#[cgp_component(ValueSerializer)]
#[derive_delegate(UseDelegate<Value>)]
pub trait CanSerializeValue<Value: ?Sized> {
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;
}
