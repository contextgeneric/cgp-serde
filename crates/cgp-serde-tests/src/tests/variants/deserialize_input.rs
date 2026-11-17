//! What `DeserializeVariantFields` accepts and rejects.

use cgp_serde::types::DeserializeWithContext;
use serde::de::DeserializeSeed;
use serde::de::value::{Error, MapAccessDeserializer, MapDeserializer};

use super::{App, Circle, Digit, Shape};
use crate::tests::support::{from_json, from_json_reader};

/// Reads a `Digit` from a one-entry map whose key the format hands over as bytes, as some binary
/// formats do for identifiers.
fn digit_from_bytes_key(key: &'static [u8], value: u64) -> Result<Digit, String> {
    let map = MapDeserializer::<_, Error>::new([(key, value)].into_iter());

    DeserializeWithContext::new(&App)
        .deserialize(MapAccessDeserializer::new(map))
        .map_err(|e| e.to_string())
}

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

#[test]
fn a_variant_name_given_as_bytes_matches() {
    assert_eq!(digit_from_bytes_key(b"D3", 3), Ok(Digit::D3(3)));
}

#[test]
fn an_unknown_variant_given_as_bytes_is_escaped_in_the_error() {
    assert_eq!(
        digit_from_bytes_key(b"D\xff", 0),
        Err(
            "unknown variant `D\\xff`, expected one of `D0`, `D1`, `D2`, `D3`, `D4`, `D5`, \
             `D6`, `D7`, `D8`, `D9`"
                .to_owned()
        )
    );
}
