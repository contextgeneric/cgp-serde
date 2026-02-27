use cgp::prelude::*;
use cgp_serde_alloc::traits::{Allocator, AllocatorComponent};

use crate::traits::HasArena;

#[cgp_impl(new AllocateWithArena)]
#[uses(HasArena<'a, Value>)]
impl<'a, Value: 'a> Allocator<'a, Value> {
    fn alloc(&self, value: Value) -> &'a mut Value {
        self.arena().alloc(value)
    }
}
