//! A record's fields follow the context's choices, so two contexts write the same record
//! differently.

use cgp::prelude::*;
use cgp_serde::components::{ValueDeserializerComponent, ValueSerializerComponent};
use cgp_serde::providers::{DeserializeRecordFields, SerializeRecordFields, UseSerde};
use cgp_serde_extra::providers::SerializeBase64;

use super::{App, Blob};
use crate::tests::support::assert_json_round_trip;

pub struct Base64App;

delegate_components! {
    Base64App {
        open {
            ValueSerializerComponent,
            ValueDeserializerComponent,
        };

        @ValueSerializerComponent.[u64, String]: UseSerde,
        @ValueSerializerComponent.Vec<u8>: SerializeBase64,
        @ValueSerializerComponent.Blob: SerializeRecordFields,

        @ValueDeserializerComponent.[u64, String]: UseSerde,
        @ValueDeserializerComponent.Vec<u8>: SerializeBase64,
        @ValueDeserializerComponent.Blob: DeserializeRecordFields,
    }
}

check_components! {
    #[check_trait(CanSerializeWithBase64)]
    Base64App {
        ValueSerializerComponent: Blob,
    }
}

check_components! {
    #[check_trait(CanDeserializeWithBase64)]
    <'de> Base64App {
        ValueDeserializerComponent: (Life<'de>, Blob),
    }
}

#[test]
fn two_contexts_encode_the_same_field_type_differently() {
    let blob = Blob {
        id: 1,
        data: vec![1, 2, 3],
    };

    assert_json_round_trip(&App, blob.clone(), r#"{"id":1,"data":"010203"}"#);
    assert_json_round_trip(&Base64App, blob, r#"{"id":1,"data":"AQID"}"#);
}
