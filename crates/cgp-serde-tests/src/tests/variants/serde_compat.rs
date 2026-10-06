//! The variant providers' output matches what Serde's derive reads and writes for the same enum.

use serde::{Deserialize, Serialize};

use super::{App, Circle, Rectangle, Shape, Token};
use crate::tests::support::{from_json, from_postcard, from_ron, to_json, to_postcard, to_ron};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct CircleMirror {
    radius: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct RectangleMirror {
    width: u64,
    height: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum ShapeMirror {
    Circle(CircleMirror),
    Rectangle(RectangleMirror),
    Label(String),
    Empty(()),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum TokenMirror {
    Word(String),
    Number(u64),
}

fn shapes() -> Vec<(Shape, ShapeMirror)> {
    vec![
        (
            Shape::Circle(Circle { radius: 3 }),
            ShapeMirror::Circle(CircleMirror { radius: 3 }),
        ),
        (
            Shape::Rectangle(Rectangle {
                width: 1,
                height: 2,
            }),
            ShapeMirror::Rectangle(RectangleMirror {
                width: 1,
                height: 2,
            }),
        ),
        (Shape::Label("x".into()), ShapeMirror::Label("x".into())),
        (Shape::Empty(()), ShapeMirror::Empty(())),
    ]
}

#[test]
fn json_agrees_with_serde_derive_in_both_directions() {
    for (shape, mirror) in shapes() {
        let ours = to_json(&App, &shape);
        assert_eq!(ours, serde_json::to_string(&mirror).unwrap());
        assert_eq!(serde_json::from_str::<ShapeMirror>(&ours).unwrap(), mirror);
        assert_eq!(from_json::<App, Shape>(&App, &ours), Ok(shape));
    }
}

#[test]
fn ron_agrees_with_serde_derive_for_non_record_payloads() {
    let ours = to_ron(&App, &Shape::Label("x".into()));
    assert_eq!(
        ours,
        ron::to_string(&ShapeMirror::Label("x".into())).unwrap()
    );

    let theirs = ron::to_string(&TokenMirror::Number(5)).unwrap();
    assert_eq!(from_ron::<App, Token>(&App, &theirs), Ok(Token::Number(5)));
}

#[test]
fn postcard_agrees_with_serde_derive_for_non_record_payloads() {
    assert_eq!(
        to_postcard(&App, &Token::Number(5)).unwrap(),
        postcard::to_allocvec(&TokenMirror::Number(5)).unwrap()
    );

    let theirs = postcard::to_allocvec(&TokenMirror::Word("hi".into())).unwrap();
    assert_eq!(
        from_postcard::<App, Token>(&App, &theirs),
        Ok(Token::Word("hi"))
    );
}
