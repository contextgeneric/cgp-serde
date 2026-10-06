//! One generic wiring entry serves every instance of a generic record.

use super::{App, Pair, Point};
use crate::tests::support::assert_json_round_trip;

#[test]
fn a_generic_entry_serves_each_instance() {
    assert_json_round_trip(
        &App,
        Pair {
            first: 1u64,
            second: 2u64,
        },
        r#"{"first":1,"second":2}"#,
    );

    assert_json_round_trip(
        &App,
        Pair {
            first: Point { x: 1, y: 2 },
            second: Point { x: 3, y: 4 },
        },
        r#"{"first":{"x":1,"y":2},"second":{"x":3,"y":4}}"#,
    );
}
