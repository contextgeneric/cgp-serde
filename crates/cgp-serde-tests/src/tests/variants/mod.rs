//! Tests for `SerializeVariantFields` and `DeserializeVariantFields`.
//!
//! The enums derive `CgpVariant` and their payload records derive `CgpData`. One context, `App`,
//! wires every type they reach; files that need a different choice define a second context. Tests
//! that pin a known issue say so, so that fixing the issue fails the test.

mod deserialize_input;
mod formats;
mod lifetimes;
mod nesting;
mod payloads;
mod round_trip;
mod serde_compat;
mod unit_payload;

use cgp::prelude::*;
use cgp_serde::components::{ValueDeserializerComponent, ValueSerializerComponent};
use cgp_serde::providers::{
    DeserializeExtend, DeserializeRecordFields, DeserializeVariantFields, SerializeDeref,
    SerializeIterator, SerializeRecordFields, SerializeVariantFields, UseSerde,
};
use cgp_serde_extra::providers::SerializeHex;

#[derive(Debug, Clone, PartialEq, CgpData)]
pub struct Circle {
    pub radius: u64,
}

#[derive(Debug, Clone, PartialEq, CgpData)]
pub struct Rectangle {
    pub width: u64,
    pub height: u64,
}

#[derive(Debug, Clone, PartialEq, CgpVariant)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
    Label(String),
    Empty(()),
}

#[derive(Debug, Clone, PartialEq, CgpVariant)]
pub enum Only {
    Only(u64),
}

#[derive(Debug, Clone, PartialEq, CgpVariant)]
pub enum Digit {
    D0(u64),
    D1(u64),
    D2(u64),
    D3(u64),
    D4(u64),
    D5(u64),
    D6(u64),
    D7(u64),
    D8(u64),
    D9(u64),
}

#[derive(Debug, Clone, PartialEq, CgpVariant)]
pub enum Payload {
    Count(u64),
    Maybe(Option<String>),
    Many(Vec<u64>),
    Bytes(Vec<u8>),
    Inner(Shape),
}

#[derive(Debug, Clone, PartialEq, CgpData)]
pub struct Drawing {
    pub title: String,
    pub shapes: Vec<Shape>,
}

#[derive(Debug, Clone, PartialEq, CgpVariant)]
pub enum Token<'a> {
    Word(&'a str),
    Number(u64),
}

pub struct App;

delegate_components! {
    App {
        open {
            ValueSerializerComponent,
            ValueDeserializerComponent,
        };

        @ValueSerializerComponent.<'a, T> &'a T:
            SerializeDeref,
        @ValueSerializerComponent.[u64, String, (), Option<String>, <'a> &'a str]:
            UseSerde,
        @ValueSerializerComponent.Vec<u8>:
            SerializeHex,
        @ValueSerializerComponent.[Vec<u64>, Vec<Shape>]:
            SerializeIterator,
        @ValueSerializerComponent.[Circle, Rectangle, Drawing]:
            SerializeRecordFields,
        @ValueSerializerComponent.[Shape, Only, Digit, Payload, Token<'static>]:
            SerializeVariantFields,

        @ValueDeserializerComponent.[u64, String, (), Option<String>, <'a> &'a str]:
            UseSerde,
        @ValueDeserializerComponent.Vec<u8>:
            SerializeHex,
        @ValueDeserializerComponent.[Vec<u64>, Vec<Shape>]:
            DeserializeExtend,
        @ValueDeserializerComponent.[Circle, Rectangle, Drawing]:
            DeserializeRecordFields,
        @ValueDeserializerComponent.[Shape, Only, Digit, Payload, <'a> Token<'a>]:
            DeserializeVariantFields,
    }
}

check_components! {
    #[check_trait(CanSerializeVariants)]
    App {
        ValueSerializerComponent: [
            Shape,
            Only,
            Digit,
            Payload,
            Drawing,
            Token<'static>,
        ],
    }
}

check_components! {
    #[check_trait(CanDeserializeVariants)]
    <'de> App {
        ValueDeserializerComponent: [
            (Life<'de>, Shape),
            (Life<'de>, Only),
            (Life<'de>, Digit),
            (Life<'de>, Payload),
            (Life<'de>, Drawing),
            (Life<'de>, Token<'de>),
        ],
    }
}
