use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;
use cgp_error_anyhow::{RaiseAnyhowError, UseAnyhowError};
use cgp_serde::components::{CanSerializeValue, ValueSerializer, ValueSerializerComponent};
use cgp_serde::providers::{SerializeDeref, SerializeIterator, UseSerde};
use cgp_serde::types::SerializeWithContext;
use cgp_serde_json::code::SerializeJson;
use cgp_serde_json::providers::SerializeToJsonString;
use serde::Serialize;
use serde::ser::{SerializeMap, SerializeSeq};

/*
    This test case tests the implementation of `ValueSerializer` for recursive data structures
    like `List`. There are 3 ways the serialization can be implemented.
*/

#[derive(Serialize)]
pub enum List<T> {
    Empty,
    Cons(T, Box<List<T>>),
}

// A manual implementation of `ValueSerializer` for `List<T>`, which serializes the items
// in a loop. There is no recursion involved, so the implementation is ok.
mod manual_impl {
    use super::*;

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
            SerializeToJsonString::try_compute(&app, PhantomData::<SerializeJson>, &values)
                .unwrap();

        assert_eq!(serialized, "[1,2]");
    }
}

// An implementation reuse by using `SerializeIterator` to serialize `List`.
// We only need to implement `IntoIterator` for `List`, and the rest is taken care of.
mod serialize_iter {
    use super::*;

    // We can also use `SerializeIterator` to implement `CanSerializeValue<List<T>>`
    // if we implement `IntoIterator` for it.
    impl<'a, T> IntoIterator for &'a List<T> {
        type Item = &'a T;
        type IntoIter = ListIterator<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            ListIterator { current: self }
        }
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
                SerializeIterator,
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
            SerializeToJsonString::try_compute(&app, PhantomData::<SerializeJson>, &values)
                .unwrap();

        assert_eq!(serialized, "[1,2]");
    }
}

// A recursive implementation of `ValueSerializer` for `List` that is automatically
// implemented using either derive macros or data-generic providers.
// The recursive lookup requires some hack to workaround the requirement overflow
// error raised by the Rust compiler.
mod serialize_recursive {
    use super::*;

    // This is a hand-rolled implementation of `ValueSerializer` implementation for `List`,
    // following what is likely generated by a derive macro.
    // We could also implement this in a data-generic way similar to `SerializeFields`.
    // However, the data-generic implementation for enums is not yet done, due to lack of
    // public interest.
    #[cgp_impl(new SerializeListAsNestedItem)]
    impl<T> ValueSerializer<List<T>>
    where
        Self: CanSerializeValue<T>,
        // The problematic part is the constraint below that eventually requires
        // `Self: CanSerializeValue<List<T>>`, which causes requirement overflow.
        Self: CanSerializeValue<Box<List<T>>>,
    {
        fn serialize<S>(&self, values: &List<T>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut serializer = serializer.serialize_map(None)?;

            // Serialize the values as a nested JSON data structure to better reflect
            // more complex recursive data structures like trees.
            match values {
                List::Empty => {
                    serializer.serialize_entry("tag", "empty")?;
                }
                List::Cons(head, tail) => {
                    serializer.serialize_entry("tag", "cons")?;

                    serializer.serialize_entry(
                        "head",
                        &SerializeWithContext {
                            context: self,
                            value: head,
                        },
                    )?;

                    serializer.serialize_entry(
                        "tail",
                        &SerializeWithContext {
                            context: self,
                            value: tail,
                        },
                    )?;
                }
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
            <T> @ValueSerializerComponent.Box<T>:
                SerializeDeref,
            <'a, T> @ValueSerializerComponent.&'a T:
                SerializeDeref,
            <T> @ValueSerializerComponent.List<T>:
                // Use a proxy provider below to enable recursive implementation.
                SerializeListForApp,
                // Directly using the recursive provider below will result in overflow:
                // SerializeListAsNestedItem,
        }
    }

    // [Workaround] Recursive implementation requires explicit proxy implementation on the concrete
    // `App` context without introducing additional `where` bounds. This way, Rust would "count"
    // this implementation as final when the same impl block is revisited recursively later on.
    #[cgp_impl(new SerializeListForApp)]
    impl<T> ValueSerializer<List<T>> for App
    where
        Self: CanSerializeValue<T>,
    {
        fn serialize<S>(&self, values: &List<T>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            // We manually forward the implementation to the recursive provider.
            // This no longer result in requirement overflow.
            SerializeListAsNestedItem::serialize(self, values, serializer)
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
            SerializeToJsonString::try_compute(&app, PhantomData::<SerializeJson>, &values)
                .unwrap();

        assert_eq!(
            serialized,
            r#"{"tag":"cons","head":1,"tail":{"tag":"cons","head":2,"tail":{"tag":"empty"}}}"#
        );
    }
}
