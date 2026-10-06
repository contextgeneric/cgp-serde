//! Known issue: a record that contains itself overflows the trait solver, because serializing it
//! requires serializing itself again.

use cgp::prelude::*;
use cgp_serde::components::ValueSerializerComponent;
use cgp_serde::providers::{SerializeDeref, SerializeIterator, SerializeRecordFields, UseSerde};

#[derive(CgpData)]
pub struct Node {
    pub id: u64,
    pub children: Vec<Node>,
}

pub struct App;

delegate_components! {
    App {
        open ValueSerializerComponent;

        @ValueSerializerComponent.<'a, T> &'a T: SerializeDeref,
        @ValueSerializerComponent.u64: UseSerde,
        @ValueSerializerComponent.Vec<Node>: SerializeIterator,
        @ValueSerializerComponent.Node: SerializeRecordFields,
    }
}

check_components! {
    App {
        ValueSerializerComponent: Node,
    }
}

fn main() {}
