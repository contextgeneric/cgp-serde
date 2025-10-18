use cgp::prelude::*;
use cgp_serde_alloc::traits::{Allocator, AllocatorComponent};

use crate::traits::HasArena;

#[cgp_impl(new AllocateWithArena)]
impl<'a, Context, Value: 'a> Allocator<'a, Value> for Context
where
    Context: HasArena<'a, Value>,
{
    fn alloc(context: &Context, value: Value) -> &'a mut Value {
        context.arena().alloc(value)
    }
}
