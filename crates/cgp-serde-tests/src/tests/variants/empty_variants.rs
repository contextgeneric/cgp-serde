//! A variant with no fields, written `Closed`, `Paused()`, or `Archived {}`, carries the payload
//! `Nil`, and the context's wiring for `Nil` decides how it is written. `App` wires `Nil` to
//! `SerializeUnit`, so each is written as a newtype variant holding `null`.

use super::{App, Status};
use crate::tests::support::{
    assert_json_round_trip, from_json, from_postcard, from_ron, to_postcard, to_ron,
};

#[test]
fn every_empty_form_holds_a_unit() {
    assert_json_round_trip(&App, Status::Closed, r#"{"Closed":null}"#);
    assert_json_round_trip(&App, Status::Paused(), r#"{"Paused":null}"#);
    assert_json_round_trip(&App, Status::Archived {}, r#"{"Archived":null}"#);
    assert_json_round_trip(&App, Status::Active(3), r#"{"Active":3}"#);
}

#[test]
fn an_empty_variant_reads_only_a_unit() {
    assert_eq!(
        from_json::<App, Status>(&App, r#"{"Closed":{}}"#),
        Err("invalid type: map, expected unit at line 1 column 10".to_owned())
    );
}

/// Serde's derive writes a unit variant as its bare name, which the provider rejects, since it
/// reads only the newtype form.
#[test]
fn a_bare_variant_name_is_rejected() {
    assert_eq!(
        from_json::<App, Status>(&App, r#""Closed""#),
        Err("invalid type: unit variant, expected newtype variant".to_owned())
    );
}

#[test]
fn ron_and_postcard_write_the_unit_payload() {
    assert_eq!(to_ron(&App, &Status::Paused()), "Paused(())");
    assert_eq!(
        from_ron::<App, Status>(&App, "Paused(())"),
        Ok(Status::Paused())
    );

    assert_eq!(to_postcard(&App, &Status::Archived {}), Ok(vec![3]));
    assert_eq!(
        from_postcard::<App, Status>(&App, &[3]),
        Ok(Status::Archived {})
    );
}
