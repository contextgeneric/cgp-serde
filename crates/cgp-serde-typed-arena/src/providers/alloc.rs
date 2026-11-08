use cgp::prelude::*;
use cgp_serde_alloc::traits::{Allocator, AllocatorComponent};

use crate::traits::HasArena;

/// Allocates a value into the arena the context's `HasArena` returns. See
/// [the reference page](https://contextgeneric.dev/docs/projects/cgp-serde/reference/providers/allocate_with_arena).
#[cgp_impl(new AllocateWithArena)]
#[uses(HasArena<'a, Value>)]
impl<'a, Value: 'a> Allocator<'a, Value> {
    fn alloc(&self, value: Value) -> &'a mut Value {
        self.arena().alloc(value)
    }
}
