//! The variant providers' output matches what Serde's derive reads and writes for the same enum.

use serde::{Deserialize, Serialize};

use super::{App, Circle, Rectangle, Shape, Status, Token};
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
enum StatusMirror {
    Active(u64),
    Closed,
    Paused(),
    Archived {},
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

/// Known issue: Serde's derive writes each empty form its own way, as a bare name, an empty
/// sequence, or an empty map, where the providers write every one as a newtype variant holding a
/// unit. The providers read none of Serde's forms. Serde reads the providers' form only for a unit
/// variant, since `serde_json` accepts `{"Closed":null}` for one.
#[test]
fn json_differs_from_serde_derive_for_empty_variants() {
    assert_eq!(
        serde_json::to_string(&StatusMirror::Closed).unwrap(),
        r#""Closed""#
    );
    assert_eq!(
        from_json::<App, Status>(&App, r#""Closed""#),
        Err("invalid type: unit variant, expected newtype variant".to_owned())
    );
    assert_eq!(
        serde_json::from_str::<StatusMirror>(&to_json(&App, &Status::Closed)).unwrap(),
        StatusMirror::Closed
    );

    assert_eq!(
        serde_json::to_string(&StatusMirror::Paused()).unwrap(),
        r#"{"Paused":[]}"#
    );
    assert_eq!(
        from_json::<App, Status>(&App, r#"{"Paused":[]}"#),
        Err("invalid type: sequence, expected unit at line 1 column 10".to_owned())
    );
    assert!(serde_json::from_str::<StatusMirror>(&to_json(&App, &Status::Paused())).is_err());

    assert_eq!(
        serde_json::to_string(&StatusMirror::Archived {}).unwrap(),
        r#"{"Archived":{}}"#
    );
    assert_eq!(
        from_json::<App, Status>(&App, r#"{"Archived":{}}"#),
        Err("invalid type: map, expected unit at line 1 column 12".to_owned())
    );
    assert!(serde_json::from_str::<StatusMirror>(&to_json(&App, &Status::Archived {})).is_err());
}

/// postcard writes a unit, an empty tuple, and an empty struct as nothing, so every empty form is
/// the bare index on both sides, and each reads the other's bytes.
#[test]
fn postcard_agrees_with_serde_derive_for_empty_variants() {
    let cases = [
        (Status::Closed, StatusMirror::Closed),
        (Status::Paused(), StatusMirror::Paused()),
        (Status::Archived {}, StatusMirror::Archived {}),
    ];

    for (status, mirror) in cases {
        let theirs = postcard::to_allocvec(&mirror).unwrap();
        assert_eq!(to_postcard(&App, &status).unwrap(), theirs);
        assert_eq!(from_postcard::<App, Status>(&App, &theirs), Ok(status));
        assert_eq!(
            postcard::from_bytes::<StatusMirror>(&theirs).unwrap(),
            mirror
        );
    }
}
