//! What `DeserializeVariantFields` accepts and rejects.

use super::{App, Circle, Shape};
use crate::tests::support::{from_json, from_json_reader};

#[test]
fn an_unknown_variant_lists_the_expected_names() {
    assert_eq!(
        from_json::<App, Shape>(&App, r#"{"Square":{"side":1}}"#),
        Err(
            "unknown variant `Square`, expected one of `Circle`, `Rectangle`, `Label`, `Empty` \
             at line 1 column 9"
                .to_owned()
        )
    );
}

#[test]
fn an_escaped_variant_name_matches() {
    assert_eq!(
        from_json::<App, Shape>(&App, r#"{"Label":"x"}"#),
        Ok(Shape::Label("x".into()))
    );
}

#[test]
fn input_read_through_io_read_is_accepted() {
    assert_eq!(
        from_json_reader::<App, Shape>(&App, r#"{"Circle":{"radius":4}}"#),
        Ok(Shape::Circle(Circle { radius: 4 }))
    );
}

#[test]
fn an_object_with_no_variant_is_rejected() {
    assert_eq!(
        from_json::<App, Shape>(&App, r#"{}"#),
        Err("expected value at line 1 column 2".to_owned())
    );
}

#[test]
fn an_object_with_two_variants_is_rejected() {
    assert_eq!(
        from_json::<App, Shape>(&App, r#"{"Label":"x","Circle":{"radius":1}}"#),
        Err("expected value at line 1 column 12".to_owned())
    );
}

#[test]
fn a_value_that_is_not_an_enum_is_rejected() {
    for input in [r#"[1,2]"#, r#"5"#, r#"null"#] {
        assert_eq!(
            from_json::<App, Shape>(&App, input),
            Err("expected value at line 1 column 1".to_owned()),
            "input: {input}"
        );
    }
}

#[test]
fn a_wrong_payload_type_names_the_payload() {
    assert_eq!(
        from_json::<App, Shape>(&App, r#"{"Label":5}"#),
        Err("invalid type: integer `5`, expected a string at line 1 column 10".to_owned())
    );
}

#[test]
fn trailing_input_is_rejected() {
    assert_eq!(
        from_json::<App, Shape>(&App, r#"{"Label":"x"} {}"#),
        Err("trailing characters at line 1 column 15".to_owned())
    );
}
