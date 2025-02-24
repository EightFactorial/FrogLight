use bevy_ecs::entity::Entity;
use bevy_reflect::func::ReflectFn;

use super::WorldRef;

pub trait BrigadierFunction<'env, Marker>: ReflectFn<'env, Marker> {}

impl<'env, Function> BrigadierFunction<'env, fn(Entity, WorldRef<'env>) -> [()]> for Function where
    Function: ReflectFn<'env, fn(Entity, WorldRef<'env>) -> [()]>
{
}

impl<'env, Function> BrigadierFunction<'env, fn(Entity, WorldRef<'env>)> for Function where
    Function: ReflectFn<'env, fn(Entity, WorldRef<'env>)>
{
}

impl<'env, Arg0, Function> BrigadierFunction<'env, fn(Arg0, Entity, WorldRef<'env>) -> [()]>
    for Function
where
    Function: ReflectFn<'env, fn(Arg0, Entity, WorldRef<'env>) -> [()]>,
{
}

impl<'env, Arg0, Function> BrigadierFunction<'env, fn(Arg0, Entity, WorldRef<'env>)> for Function where
    Function: ReflectFn<'env, fn(Arg0, Entity, WorldRef<'env>)>
{
}

impl<'env, Arg0, Arg1, Function>
    BrigadierFunction<'env, fn(Arg0, Arg1, Entity, WorldRef<'env>) -> [()]> for Function
where
    Function: ReflectFn<'env, fn(Arg0, Arg1, Entity, WorldRef<'env>) -> [()]>,
{
}

impl<'env, Arg0, Arg1, Function> BrigadierFunction<'env, fn(Arg0, Arg1, Entity, WorldRef<'env>)>
    for Function
where
    Function: ReflectFn<'env, fn(Arg0, Arg1, Entity, WorldRef<'env>)>,
{
}
