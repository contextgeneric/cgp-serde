use cgp::prelude::*;
use serde::ser::SerializeSeq;

use crate::components::{CanSerializeValue, ValueSerializer, ValueSerializerComponent};
use crate::types::SerializeWithContext;

#[cgp_impl(new SerializeIterator)]
impl<Context, Value> ValueSerializer<Value> for Context
where
    for<'a> &'a Value: IntoIterator,
    Context: for<'a> CanSerializeValue<<&'a Value as IntoIterator>::Item>,
{
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let items = value.into_iter();
        let mut serializer = serializer.serialize_seq(None)?;
        for item in items {
            serializer.serialize_element(&SerializeWithContext {
                context: self,
                value: &item,
            })?
        }

        serializer.end()
    }
}
