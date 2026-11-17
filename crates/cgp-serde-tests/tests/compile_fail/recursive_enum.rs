//! Known issue: an enum that contains itself overflows the trait solver, because serializing it
//! requires serializing itself again.

use cgp::prelude::*;
use cgp_serde::components::ValueSerializerComponent;
use cgp_serde::providers::{SerializeDeref, SerializeVariantFields, UseSerde};

#[derive(CgpVariant)]
pub enum Expr {
    Literal(u64),
    Negate(Box<Expr>),
}

pub struct App;

delegate_components! {
    App {
        open ValueSerializerComponent;

        @ValueSerializerComponent.u64: UseSerde,
        @ValueSerializerComponent.Box<Expr>: SerializeDeref,
        @ValueSerializerComponent.Expr: SerializeVariantFields,
    }
}

check_components! {
    App {
        ValueSerializerComponent: Expr,
    }
}

fn main() {}
