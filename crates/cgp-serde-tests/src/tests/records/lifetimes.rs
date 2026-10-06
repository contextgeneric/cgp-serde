//! Records with a lifetime work in both directions, borrowing from the input when the
//! deserializer can lend it.

use cgp_serde::types::DeserializeWithContext;
use serde::de::DeserializeSeed;

use super::{App, Named};
use crate::tests::support::{from_json, to_json};

#[test]
fn a_record_that_borrows_serializes() {
    let name = String::from("n");
    let tag = String::from("t");
    let record = Named {
        name: &name,
        tags: vec![&tag, &tag],
    };

    assert_eq!(to_json(&App, &record), r#"{"name":"n","tags":["t","t"]}"#);
}

#[test]
fn a_record_that_borrows_reads_from_a_string() {
    let input = String::from(r#"{"name":"x","tags":["a","b"]}"#);
    let record: Named<'_> = from_json(&App, &input).unwrap();

    assert_eq!(
        record,
        Named {
            name: "x",
            tags: vec!["a", "b"],
        }
    );
}

#[test]
fn a_borrowed_field_cannot_be_read_from_an_io_reader() {
    let mut deserializer =
        serde_json::Deserializer::from_reader(r#"{"name":"x","tags":[]}"#.as_bytes());
    let result: Result<Named<'_>, _> =
        DeserializeWithContext::new(&App).deserialize(&mut deserializer);

    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid type: string \"x\", expected a borrowed string at line 1 column 11"
    );
}
