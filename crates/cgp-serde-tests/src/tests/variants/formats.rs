//! The variant providers with RON and postcard.

use super::{App, Circle, Payload, Shape, Token};
use crate::tests::support::{from_postcard, from_ron, to_postcard, to_ron};

#[test]
fn ron_writes_the_variant_name_around_its_payload() {
    let label = Shape::Label("x".into());
    assert_eq!(to_ron(&App, &label), r#"Label("x")"#);
    assert_eq!(from_ron::<App, Shape>(&App, r#"Label("x")"#), Ok(label));

    let circle = Shape::Circle(Circle { radius: 2 });
    assert_eq!(to_ron(&App, &circle), r#"Circle({"radius":2})"#);
    assert_eq!(
        from_ron::<App, Shape>(&App, r#"Circle({"radius":2})"#),
        Ok(circle)
    );
}

/// RON rejects an enum name that is not an identifier, so the providers must pass a generic
/// enum's name without its arguments.
#[test]
fn ron_accepts_a_generic_enum() {
    assert_eq!(to_ron(&App, &Token::Number(5)), "Number(5)");
    assert_eq!(
        from_ron::<App, Token>(&App, "Number(5)"),
        Ok(Token::Number(5))
    );
}

#[test]
fn postcard_writes_the_declaration_index() {
    assert_eq!(to_postcard(&App, &Token::Number(5)), Ok(vec![1, 5]));
    assert_eq!(
        from_postcard::<App, Token>(&App, &[1, 5]),
        Ok(Token::Number(5))
    );
    assert_eq!(
        from_postcard::<App, Token>(&App, &[0, 2, b'h', b'i']),
        Ok(Token::Word("hi"))
    );
}

#[test]
fn postcard_rejects_an_index_out_of_range() {
    assert_eq!(
        from_postcard::<App, Token>(&App, &[9, 5]),
        Err(postcard::Error::SerdeDeCustom)
    );
}

/// Known issue: a record payload is written without a length, which postcard requires.
#[test]
fn postcard_rejects_a_record_payload() {
    assert_eq!(
        to_postcard(&App, &Payload::Inner(Shape::Circle(Circle { radius: 1 }))),
        Err(postcard::Error::SerializeSeqLengthUnknown)
    );
}
