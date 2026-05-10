use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;
use cgp_error_anyhow::{RaiseAnyhowError, UseAnyhowError};
use cgp_serde::components::{CanSerializeValue, ValueSerializer, ValueSerializerComponent};
use cgp_serde::providers::{SerializeDeref, UseSerde};
use cgp_serde::types::SerializeWithContext;
use cgp_serde_json::code::SerializeJson;
use cgp_serde_json::providers::SerializeToJsonString;
use serde::ser::SerializeSeq;

pub enum List<T> {
    Empty,
    Cons(T, Box<List<T>>),
}

pub struct ListIterator<'a, T> {
    current: &'a List<T>,
}

impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            List::Empty => None,
            List::Cons(head, tail) => {
                self.current = tail;
                Some(head)
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = ListIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator { current: self }
    }
}

#[cgp_impl(new SerializeList)]
impl<T> ValueSerializer<List<T>>
where
    Self: for<'a> CanSerializeValue<&'a T>,
{
    fn serialize<S>(&self, values: &List<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serializer = serializer.serialize_seq(None)?;
        let mut current = values;

        while let List::Cons(item, tail) = current {
            serializer.serialize_element(&SerializeWithContext {
                context: self,
                value: &item,
            })?;

            current = tail;
        }

        serializer.end()
    }
}

pub struct App;

delegate_components! {
    App {
        open ValueSerializerComponent;

        ErrorTypeProviderComponent:
            UseAnyhowError,
        ErrorRaiserComponent:
            RaiseAnyhowError,
        @ValueSerializerComponent.u64:
            UseSerde,
        <'a, T> @ValueSerializerComponent.&'a T:
             SerializeDeref,
        <T> @ValueSerializerComponent.List<T>:
            // We can also use `SerializeIterator` here,
             SerializeList,
    }
}

check_components! {
    App {
        ValueSerializerComponent: [
            u64,
            List<u64>,
        ],
    }
}

#[test]
fn test_list_serialization() {
    let app = App;

    let values = List::Cons(1u64, Box::new(List::Cons(2, Box::new(List::Empty))));

    let serialized =
        SerializeToJsonString::try_compute(&app, PhantomData::<SerializeJson>, &values).unwrap();

    assert_eq!(serialized, "[1,2]");
}
