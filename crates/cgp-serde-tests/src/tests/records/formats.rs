//! The record providers with formats other than JSON.

use super::{App, Point};
use crate::tests::support::{from_ron, to_postcard, to_ron};

/// Known issue: records reach the format as maps, so RON writes map syntax rather than its
/// struct syntax `(x:1,y:2)`.
#[test]
fn ron_writes_a_record_as_a_map() {
    let point = Point { x: 1, y: 2 };
    let ron = to_ron(&App, &point);

    assert_eq!(ron, r#"{"x":1,"y":2}"#);
    assert_eq!(from_ron::<App, Point>(&App, &ron), Ok(point));
}

/// Known issue: the map is opened without a length, which postcard requires.
#[test]
fn postcard_rejects_a_record() {
    assert_eq!(
        to_postcard(&App, &Point { x: 1, y: 2 }),
        Err(postcard::Error::SerializeSeqLengthUnknown)
    );
}
