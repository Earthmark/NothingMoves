use bevy::{prelude::*, tasks::Task};
use futures_lite::future;

pub fn promote_task_component<Comp: Component>(
    mut commands: Commands,
    mut task_waiter: Query<(Entity, &mut Task<Comp>)>,
) {
    for (entity, mut task) in task_waiter.iter_mut() {
        if let Some(comp) = future::block_on(future::poll_once(&mut *task)) {
            // Remote the task variant and replace it with the concrete variant.
            commands.entity(entity).remove::<Task<Comp>>().insert(comp);
        }
    }
}
