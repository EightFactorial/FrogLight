use bevy_ecs::component::Component;
use bevy_tasks::Task;

/// A task that holds a [`Connection`].
#[derive(Debug, Component)]
pub struct ConnectionTask(Task<()>);

impl ConnectionTask {
    pub(crate) fn new(task: Task<()>) -> Self { Self(task) }
}
