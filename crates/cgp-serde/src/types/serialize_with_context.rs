use crate::components::CanSerializeValue;

/// Wraps a value with the context, so Serde serializes it through the context's wiring. See
/// [the reference page](https://contextgeneric.dev/docs/projects/cgp-serde/reference/types/serialize_with_context).
pub struct SerializeWithContext<'a, Context, T> {
    pub context: &'a Context,
    pub value: &'a T,
}

impl<'a, Context, T> SerializeWithContext<'a, Context, T> {
    pub fn new(context: &'a Context, value: &'a T) -> Self {
        Self { context, value }
    }
}

impl<'a, Context, T> serde::Serialize for SerializeWithContext<'a, Context, T>
where
    Context: CanSerializeValue<T>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.context.serialize(self.value, serializer)
    }
}
