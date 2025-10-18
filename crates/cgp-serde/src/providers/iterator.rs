use cgp::prelude::*;
use serde::ser::SerializeSeq;

use crate::components::{CanSerializeValue, ValueSerializer, ValueSerializerComponent};
use crate::types::SerializeWithContext;

pub struct SerializeIterator;

#[cgp_impl(SerializeIterator)]
impl<Context, Value> ValueSerializer<Value> for Context
where
    for<'a> &'a Value: IntoIterator,
    Context: for<'a> CanSerializeValue<<&'a Value as IntoIterator>::Item>,
{
    fn serialize<S>(context: &Context, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let items = value.into_iter();
        let mut serializer = serializer.serialize_seq(None)?;
        for item in items {
            serializer.serialize_element(&SerializeWithContext {
                context,
                value: &item,
            })?
        }

        serializer.end()
    }
}
