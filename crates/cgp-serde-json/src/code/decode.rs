use core::marker::PhantomData;

/// Names the JSON-decoding operation for `T` that a context keys its `TryComputer` wiring on.
pub struct DeserializeJson<T>(pub PhantomData<T>);
