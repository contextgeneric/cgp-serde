//! What `DeserializeRecordFields` accepts and rejects.

use super::{App, Empty, Leaves, Point, Polygon};
use crate::tests::support::{from_json, from_json_reader};

#[test]
fn keys_may_come_in_any_order() {
    assert_eq!(
        from_json::<App, Point>(&App, r#"{"y":2,"x":1}"#),
        Ok(Point { x: 1, y: 2 })
    );
}

#[test]
fn unknown_keys_are_skipped_whatever_their_value() {
    assert_eq!(
        from_json::<App, Point>(
            &App,
            r#"{"x":1,"extra":{"deep":[1,{"a":null}]},"y":2,"more":"text"}"#
        ),
        Ok(Point { x: 1, y: 2 })
    );

    assert_eq!(
        from_json::<App, Empty>(&App, r#"{"anything":[1,2,3]}"#),
        Ok(Empty {})
    );
}

#[test]
fn escaped_keys_match_their_field() {
    assert_eq!(
        from_json::<App, Point>(&App, r#"{"x":1,"y":2}"#),
        Ok(Point { x: 1, y: 2 })
    );
}

#[test]
fn input_read_through_io_read_is_accepted() {
    assert_eq!(
        from_json_reader::<App, Polygon>(
            &App,
            r#"{"name":"p","origin":{"x":1,"y":1},"vertices":[{"x":2,"y":3}]}"#
        ),
        Ok(Polygon {
            name: "p".into(),
            origin: Point { x: 1, y: 1 },
            vertices: vec![Point { x: 2, y: 3 }],
        })
    );
}

#[test]
fn a_missing_field_is_reported_after_the_map() {
    assert_eq!(
        from_json::<App, Point>(&App, r#"{"x":1}"#),
        Err("missing field: y at line 1 column 7".to_owned())
    );
}

/// Known issue: a missing `Option` field is an error rather than `None`; see "A missing field
/// is always an error" in the knowledge base's cgp-serde records reference.
#[test]
fn a_missing_option_field_is_an_error() {
    assert_eq!(
        from_json::<App, Leaves>(&App, r#"{"flag":true,"count":1,"delta":0,"label":"x"}"#),
        Err("missing field: note at line 1 column 45".to_owned())
    );
}

#[test]
fn a_duplicate_field_is_rejected() {
    assert_eq!(
        from_json::<App, Point>(&App, r#"{"x":1,"x":2,"y":3}"#),
        Err("duplicate field: x at line 1 column 12".to_owned())
    );
}

/// Known issue: the sequence form Serde's derive also accepts is rejected.
#[test]
fn a_sequence_is_rejected() {
    assert_eq!(
        from_json::<App, Point>(&App, r#"[1,2]"#),
        Err("invalid type: sequence, expected map at line 1 column 0".to_owned())
    );
}

#[test]
fn a_wrong_field_type_names_the_value() {
    assert_eq!(
        from_json::<App, Point>(&App, r#"{"x":"one","y":2}"#),
        Err("invalid type: string \"one\", expected u64 at line 1 column 10".to_owned())
    );
}

#[test]
fn trailing_input_is_rejected() {
    assert_eq!(
        from_json::<App, Point>(&App, r#"{"x":1,"y":2} {}"#),
        Err("trailing characters at line 1 column 15".to_owned())
    );
}
