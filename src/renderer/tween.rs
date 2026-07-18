use bevy::prelude::*;
use std::time::Duration;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, tick_tweens);
}

#[derive(Component)]
#[require(Transform)]
pub struct TransformTween {
    timer: Timer,
    ease: EaseFunction,
    start: Transform,
    end: Transform,
    on_complete: OnComplete,
}

#[derive(Default)]
pub enum OnComplete {
    #[default]
    Keep,
    Despawn,
}

impl TransformTween {
    pub fn new(duration: Duration, start: Transform, end: Transform) -> Self {
        Self {
            timer: Timer::new(duration, TimerMode::Once),
            ease: EaseFunction::CubicInOut,
            start,
            end,
            on_complete: OnComplete::Keep,
        }
    }

    pub fn despawn_on_complete(mut self) -> Self {
        self.on_complete = OnComplete::Despawn;
        self
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
            match tween.on_complete {
                OnComplete::Keep => {
                    c.entity(e).remove::<TransformTween>();
                }
                OnComplete::Despawn => {
                    c.entity(e).despawn();
                }
            }
        }
    }
}
