use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use bevy_reflect::func::{ReflectFn, args::FromArg};

pub struct FunctionBuilder<'env, Function: ReflectFn<'env, Marker>, Marker> {
    _e: PhantomData<&'env Marker>,
    marker: PhantomData<Function>,
}

impl<'env, Function: ReflectFn<'env, Marker>, Marker> FunctionBuilder<'env, Function, Marker> {
    fn register(&mut self, _function: Function) {}
}

impl<'env, Function: ReflectFn<'env, Marker> + Fn(Entity, String), Marker>
    FunctionBuilder<'env, Function, Marker>
{
    #[must_use]
    fn new() -> Self { FunctionBuilder { _e: PhantomData, marker: PhantomData } }

    #[must_use]
    fn add_field<
        NewArg: FromArg,
        NewFunction: ReflectFn<'env, fn(NewArg, Entity, String) -> [()]> + Fn(NewArg, Entity, String),
    >(
        self,
    ) -> FunctionBuilder<'env, NewFunction, fn(NewArg, Entity, String) -> [()]> {
        FunctionBuilder { _e: PhantomData, marker: PhantomData }
    }
}

#[test]
#[ignore]
fn test() {
    // Create a new function builder
    let mut builder = FunctionBuilder::new();
    builder.register(|_entity, _string| {});

    // Add a field to the builder
    let mut builder = builder.add_field::<i32, _>();
    builder.register(|_integer, _entity, _string| {});
}
