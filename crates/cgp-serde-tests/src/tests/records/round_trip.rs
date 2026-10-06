//! Records serialize as maps in declaration order and read back to the same value.

use super::{App, Empty, Leaves, Point, Polygon, Single};
use crate::tests::support::assert_json_round_trip;

#[test]
fn fields_are_written_in_declaration_order() {
    assert_json_round_trip(&App, Point { x: 1, y: 2 }, r#"{"x":1,"y":2}"#);
}

#[test]
fn a_single_field_record_is_still_a_map() {
    assert_json_round_trip(
        &App,
        Single {
            value: "one".into(),
        },
        r#"{"value":"one"}"#,
    );
}

#[test]
fn an_empty_record_is_an_empty_map() {
    assert_json_round_trip(&App, Empty {}, r#"{}"#);
}

#[test]
fn each_leaf_type_is_written_by_its_own_entry() {
    assert_json_round_trip(
        &App,
        Leaves {
            flag: true,
            count: 7,
            delta: -3,
            label: "x".into(),
            note: Some("n".into()),
        },
        r#"{"flag":true,"count":7,"delta":-3,"label":"x","note":"n"}"#,
    );

    assert_json_round_trip(
        &App,
        Leaves {
            flag: false,
            count: 0,
            delta: 0,
            label: String::new(),
            note: None,
        },
        r#"{"flag":false,"count":0,"delta":0,"label":"","note":null}"#,
    );
}

#[test]
fn leaf_edge_values_survive_the_round_trip() {
    assert_json_round_trip(
        &App,
        Leaves {
            flag: true,
            count: u64::MAX,
            delta: i64::MIN,
            label: "h\u{e9}llo \"quoted\"\n\ttab \u{1f980}".into(),
            note: Some("\\".into()),
        },
        "{\"flag\":true,\"count\":18446744073709551615,\"delta\":-9223372036854775808,\
         \"label\":\"h\u{e9}llo \\\"quoted\\\"\\n\\ttab \u{1f980}\",\"note\":\"\\\\\"}",
    );
}

#[test]
fn nested_records_and_collections_of_records_follow_the_context() {
    assert_json_round_trip(
        &App,
        Polygon {
            name: "tri".into(),
            origin: Point { x: 0, y: 0 },
            vertices: vec![Point { x: 1, y: 2 }, Point { x: 3, y: 4 }],
        },
        r#"{"name":"tri","origin":{"x":0,"y":0},"vertices":[{"x":1,"y":2},{"x":3,"y":4}]}"#,
    );

    assert_json_round_trip(
        &App,
        Polygon {
            name: "none".into(),
            origin: Point { x: 5, y: 5 },
            vertices: vec![],
        },
        r#"{"name":"none","origin":{"x":5,"y":5},"vertices":[]}"#,
    );
}
