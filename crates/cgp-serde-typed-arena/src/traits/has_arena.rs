use cgp::prelude::*;
use typed_arena::Arena;

#[cgp_getter(ArenaGetter)]
#[derive_delegate(UseDelegate<T>)]
pub trait HasArena<'a, T: 'a> {
    fn arena(&self) -> &&'a Arena<T>;
}
