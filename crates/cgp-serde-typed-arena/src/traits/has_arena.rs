use cgp::prelude::*;
use typed_arena::Arena;

/// Reads the `typed_arena::Arena<T>` the context allocates `T` values into.
#[cgp_getter(ArenaGetter)]
#[derive_delegate(UseDelegate<T>)]
pub trait HasArena<'a, T: 'a> {
    fn arena(&self) -> &&'a Arena<T>;
}
