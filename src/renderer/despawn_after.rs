use bevy::prelude::*;
use std::time::Duration;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, despawn_counter);
}

#[derive(Component)]
pub struct DespawnAfter(Timer);

impl DespawnAfter {
    pub fn new(dur: Duration) -> Self {
        Self(Timer::new(dur, TimerMode::Once))
    }
}

fn despawn_counter(time: Res<Time>, mut c: Commands, q: Query<(Entity, &mut DespawnAfter)>) {
    for (e, mut d) in q {
        if d.0.tick(time.delta()).just_finished() {
            c.entity(e).try_despawn();
        }
    }
}
