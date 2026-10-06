//! Known issue: `SerializeVariantFields` accepts only `'static` enums, because its bound on the
//! enum's borrowed field view must hold for every lifetime.

use cgp::prelude::*;
use cgp_serde::components::ValueSerializerComponent;
use cgp_serde::providers::{SerializeVariantFields, UseSerde};

#[derive(CgpVariant)]
pub enum Token<'a> {
    Word(&'a str),
    Number(u64),
}

pub struct App;

delegate_components! {
    App {
        open ValueSerializerComponent;

        @ValueSerializerComponent.[u64, <'a> &'a str]: UseSerde,
        @ValueSerializerComponent.<'a> Token<'a>: SerializeVariantFields,
    }
}

check_components! {
    <'a> App {
        ValueSerializerComponent: Token<'a>,
    }
}

fn main() {}
