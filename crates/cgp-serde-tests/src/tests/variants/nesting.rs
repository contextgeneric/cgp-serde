//! Enums inside records and collections, and a payload whose encoding differs by context.

use cgp::prelude::*;
use cgp_serde::components::{ValueDeserializerComponent, ValueSerializerComponent};
use cgp_serde::providers::{
    DeserializeExtend, DeserializeRecordFields, DeserializeVariantFields, SerializeDeref,
    SerializeIterator, SerializeRecordFields, SerializeVariantFields, UseSerde,
};
use cgp_serde_extra::providers::SerializeBase64;

use super::{App, Circle, Drawing, Payload, Rectangle, Shape};
use crate::tests::support::assert_json_round_trip;

pub struct Base64App;

delegate_components! {
    Base64App {
        open {
            ValueSerializerComponent,
            ValueDeserializerComponent,
        };

        @ValueSerializerComponent.<'a, T> &'a T: SerializeDeref,
        @ValueSerializerComponent.[u64, String, (), Option<String>]: UseSerde,
        @ValueSerializerComponent.Vec<u8>: SerializeBase64,
        @ValueSerializerComponent.Vec<u64>: SerializeIterator,
        @ValueSerializerComponent.[Circle, Rectangle]: SerializeRecordFields,
        @ValueSerializerComponent.[Shape, Payload]: SerializeVariantFields,

        @ValueDeserializerComponent.[u64, String, (), Option<String>]: UseSerde,
        @ValueDeserializerComponent.Vec<u8>: SerializeBase64,
        @ValueDeserializerComponent.Vec<u64>: DeserializeExtend,
        @ValueDeserializerComponent.[Circle, Rectangle]: DeserializeRecordFields,
        @ValueDeserializerComponent.[Shape, Payload]: DeserializeVariantFields,
    }
}

check_components! {
    #[check_trait(CanSerializeWithBase64)]
    Base64App {
        ValueSerializerComponent: Payload,
    }
}

check_components! {
    #[check_trait(CanDeserializeWithBase64)]
    <'de> Base64App {
        ValueDeserializerComponent: (Life<'de>, Payload),
    }
}

#[test]
fn enums_inside_a_record_and_a_collection() {
    assert_json_round_trip(
        &App,
        Drawing {
            title: "t".into(),
            shapes: vec![
                Shape::Label("a".into()),
                Shape::Empty(()),
                Shape::Circle(Circle { radius: 2 }),
            ],
        },
        r#"{"title":"t","shapes":[{"Label":"a"},{"Empty":null},{"Circle":{"radius":2}}]}"#,
    );
}

#[test]
fn two_contexts_encode_the_same_payload_differently() {
    let payload = Payload::Bytes(vec![1, 2, 3]);

    assert_json_round_trip(&App, payload.clone(), r#"{"Bytes":"010203"}"#);
    assert_json_round_trip(&Base64App, payload, r#"{"Bytes":"AQID"}"#);
}
