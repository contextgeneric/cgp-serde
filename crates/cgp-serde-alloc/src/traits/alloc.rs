use cgp::prelude::*;

/// Moves a value into storage that lives for `'a` and returns a reference to it. See
/// [the reference page](https://contextgeneric.dev/docs/projects/cgp-serde/reference/components/can_alloc).
#[cgp_component(Allocator)]
pub trait CanAlloc<'a, T> {
    fn alloc(&self, value: T) -> &'a mut T;
}
