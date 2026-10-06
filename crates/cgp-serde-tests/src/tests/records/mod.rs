//! Tests for `SerializeRecordFields` and `DeserializeRecordFields`.
//!
//! The data types derive only `CgpData`, and one context, `App`, wires every type they reach.
//! Each file tests one concern. Tests that pin a known issue say so, so that fixing the issue
//! fails the test and points at the issue to remove.

mod context_choice;
mod deserialize_input;
mod formats;
mod generic;
mod lifetimes;
mod round_trip;
mod serde_compat;

use cgp::prelude::*;
use cgp_serde::components::{ValueDeserializerComponent, ValueSerializerComponent};
use cgp_serde::providers::{
    DeserializeExtend, DeserializeRecordFields, SerializeDeref, SerializeIterator,
    SerializeRecordFields, UseSerde,
};
use cgp_serde_extra::providers::SerializeHex;

#[derive(Debug, Clone, PartialEq, CgpData)]
pub struct Point {
    pub x: u64,
    pub y: u64,
}

#[derive(Debug, Clone, PartialEq, CgpData)]
pub struct Single {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, CgpData)]
pub struct Empty {}

#[derive(Debug, Clone, PartialEq, CgpData)]
pub struct Leaves {
    pub flag: bool,
    pub count: u64,
    pub delta: i64,
    pub label: String,
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, CgpData)]
pub struct Polygon {
    pub name: String,
    pub origin: Point,
    pub vertices: Vec<Point>,
}

#[derive(Debug, Clone, PartialEq, CgpData)]
pub struct Blob {
    pub id: u64,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, CgpData)]
pub struct Named<'a> {
    pub name: &'a str,
    pub tags: Vec<&'a str>,
}

#[derive(Debug, Clone, PartialEq, CgpData)]
pub struct Pair<T> {
    pub first: T,
    pub second: T,
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
        @ValueSerializerComponent.[bool, u64, i64, String, Option<String>, <'a> &'a str]:
            UseSerde,
        @ValueSerializerComponent.Vec<u8>:
            SerializeHex,
        @ValueSerializerComponent.[Vec<Point>, <'a> Vec<&'a str>]:
            SerializeIterator,
        @ValueSerializerComponent.[
            Point,
            Single,
            Empty,
            Leaves,
            Polygon,
            Blob,
            <'a> Named<'a>,
            <T> Pair<T>,
        ]:
            SerializeRecordFields,

        @ValueDeserializerComponent.[bool, u64, i64, String, Option<String>, <'a> &'a str]:
            UseSerde,
        @ValueDeserializerComponent.Vec<u8>:
            SerializeHex,
        @ValueDeserializerComponent.[Vec<Point>, <'a> Vec<&'a str>]:
            DeserializeExtend,
        @ValueDeserializerComponent.[
            Point,
            Single,
            Empty,
            Leaves,
            Polygon,
            Blob,
            <'a> Named<'a>,
            <T> Pair<T>,
        ]:
            DeserializeRecordFields,
    }
}

check_components! {
    #[check_trait(CanSerializeRecords)]
    <'a> App {
        ValueSerializerComponent: [
            Point,
            Single,
            Empty,
            Leaves,
            Polygon,
            Blob,
            Named<'a>,
            Pair<u64>,
            Pair<Point>,
        ],
    }
}

check_components! {
    #[check_trait(CanDeserializeRecords)]
    <'de> App {
        ValueDeserializerComponent: [
            (Life<'de>, Point),
            (Life<'de>, Single),
            (Life<'de>, Empty),
            (Life<'de>, Leaves),
            (Life<'de>, Polygon),
            (Life<'de>, Blob),
            (Life<'de>, Named<'de>),
            (Life<'de>, Pair<u64>),
            (Life<'de>, Pair<Point>),
        ],
    }
}
