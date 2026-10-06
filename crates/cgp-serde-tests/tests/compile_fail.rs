//! Wiring that cgp-serde rejects at compile time, pinned with its exact diagnostics.
//!
//! Each case is a known limitation of the generic providers rather than a mistake to fix in the
//! case. Regenerate the `.stderr` files with `TRYBUILD=overwrite` after a toolchain change.

#[test]
fn compile_fail() {
    trybuild::TestCases::new().compile_fail("tests/compile_fail/*.rs");
}
