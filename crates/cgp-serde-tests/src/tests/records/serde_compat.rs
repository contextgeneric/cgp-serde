//! The record providers' JSON matches what Serde's derive reads and writes for the same struct.

use serde::{Deserialize, Serialize};

use super::{App, Polygon};
use crate::tests::support::{from_json, to_json};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PointMirror {
    x: u64,
    y: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PolygonMirror {
    name: String,
    origin: PointMirror,
    vertices: Vec<PointMirror>,
}

fn polygon() -> Polygon {
    use super::Point;

    Polygon {
        name: "tri".into(),
        origin: Point { x: 0, y: 1 },
        vertices: vec![Point { x: 2, y: 3 }],
    }
}

fn polygon_mirror() -> PolygonMirror {
    PolygonMirror {
        name: "tri".into(),
        origin: PointMirror { x: 0, y: 1 },
        vertices: vec![PointMirror { x: 2, y: 3 }],
    }
}

#[test]
fn serde_derive_reads_what_the_provider_writes() {
    let json = to_json(&App, &polygon());
    let mirror: PolygonMirror = serde_json::from_str(&json).unwrap();

    assert_eq!(mirror, polygon_mirror());
}

#[test]
fn the_provider_reads_what_serde_derive_writes() {
    let json = serde_json::to_string(&polygon_mirror()).unwrap();

    assert_eq!(from_json::<App, Polygon>(&App, &json), Ok(polygon()));
}
