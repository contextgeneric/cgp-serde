//! Deserializing an enum can borrow from the input. Serializing needs a `'static` enum, a known
//! issue whose compile failure is pinned by the `compile_fail` UI tests.

use super::{App, Token};
use crate::tests::support::{from_json, to_json};

#[test]
fn a_borrowed_payload_reads_from_a_string() {
    let input = String::from(r#"{"Word":"hello"}"#);
    let token: Token<'_> = from_json(&App, &input).unwrap();

    assert_eq!(token, Token::Word("hello"));
}

#[test]
fn a_static_instance_of_an_enum_with_a_lifetime_serializes() {
    assert_eq!(
        to_json(&App, &Token::Word("static")),
        r#"{"Word":"static"}"#
    );
    assert_eq!(to_json(&App, &Token::Number(5)), r#"{"Number":5}"#);
}
