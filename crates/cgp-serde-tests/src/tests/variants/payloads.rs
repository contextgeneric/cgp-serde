//! A payload is any type the context wires, and it follows the context's choice for that type.

use super::{App, Circle, Payload, Shape};
use crate::tests::support::assert_json_round_trip;

#[test]
fn scalar_option_and_collection_payloads() {
    assert_json_round_trip(&App, Payload::Count(1), r#"{"Count":1}"#);
    assert_json_round_trip(&App, Payload::Maybe(Some("x".into())), r#"{"Maybe":"x"}"#);
    assert_json_round_trip(&App, Payload::Maybe(None), r#"{"Maybe":null}"#);
    assert_json_round_trip(&App, Payload::Many(vec![1, 2, 3]), r#"{"Many":[1,2,3]}"#);
    assert_json_round_trip(&App, Payload::Many(vec![]), r#"{"Many":[]}"#);
}

#[test]
fn a_payload_follows_the_context_choice_for_its_type() {
    assert_json_round_trip(&App, Payload::Bytes(vec![1, 2, 3]), r#"{"Bytes":"010203"}"#);
}

#[test]
fn an_enum_payload_is_itself_tagged() {
    assert_json_round_trip(
        &App,
        Payload::Inner(Shape::Circle(Circle { radius: 1 })),
        r#"{"Inner":{"Circle":{"radius":1}}}"#,
    );
}
