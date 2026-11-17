//! Each variant serializes as `{"Variant": payload}` and reads back to the same value.

use super::{App, Circle, Digit, Only, Rectangle, Shape};
use crate::tests::support::assert_json_round_trip;

#[test]
fn every_variant_is_tagged_with_its_name() {
    assert_json_round_trip(
        &App,
        Shape::Circle(Circle { radius: 3 }),
        r#"{"Circle":{"radius":3}}"#,
    );
    assert_json_round_trip(
        &App,
        Shape::Rectangle(Rectangle {
            width: 2,
            height: 5,
        }),
        r#"{"Rectangle":{"width":2,"height":5}}"#,
    );
    assert_json_round_trip(&App, Shape::Label("hi".into()), r#"{"Label":"hi"}"#);
    assert_json_round_trip(&App, Shape::Empty(()), r#"{"Empty":null}"#);
}

#[test]
fn a_one_variant_enum_works() {
    assert_json_round_trip(&App, Only::Only(7), r#"{"Only":7}"#);
}

#[test]
fn the_first_middle_and_last_of_many_variants_are_found() {
    assert_json_round_trip(&App, Digit::D0(0), r#"{"D0":0}"#);
    assert_json_round_trip(&App, Digit::D5(5), r#"{"D5":5}"#);
    assert_json_round_trip(&App, Digit::D9(9), r#"{"D9":9}"#);
}
