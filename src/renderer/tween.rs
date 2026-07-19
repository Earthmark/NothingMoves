use bevy::prelude::*;
use std::time::Duration;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, tick_tweens.in_set(TweenTick));
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TweenTick;

#[derive(Component)]
#[require(Transform)]
pub struct TransformTween {
    timer: Timer,
    ease: EaseFunction,
    start: Transform,
    end: Transform,
}

impl TransformTween {
    pub fn new(duration: Duration, start: Transform, end: Transform) -> Self {
        Self {
            timer: Timer::new(duration, TimerMode::Once),
            ease: EaseFunction::CubicInOut,
            start,
            end,
        }
    }

    fn sample(&self) -> Transform {
        let n = self.ease.sample_clamped(self.timer.fraction());
        Transform {
            translation: self.start.translation.lerp(self.end.translation, n),
            rotation: self.start.rotation.slerp(self.end.rotation, n),
            scale: self.start.scale.lerp(self.end.scale, n),
        }
    }
}

fn tick_tweens(
    time: Res<Time>,
    mut c: Commands,
    tweens: Query<(Entity, &mut TransformTween, &mut Transform)>,
) {
    for (e, mut tween, mut transform) in tweens {
        tween.timer.tick(time.delta());
        transform.set_if_neq(tween.sample());

        if tween.timer.is_finished() {
            c.entity(e).try_remove::<TransformTween>();
        }
    }
}
