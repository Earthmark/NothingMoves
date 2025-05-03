use bevy::prelude::*;

pub fn visible_in_plugin<States>(app: &mut App)
where
    States: bevy::prelude::States,
{
    app.add_systems(
        Update,
        update_visibility::<States>.run_if(state_exists::<States>.and(state_changed::<States>)),
    );
}

#[derive(Component)]
pub struct VisibleIn<States>(States);

impl<States> VisibleIn<States> {
    pub fn new(state: States) -> Self {
        Self(state)
    }
}

fn update_visibility<States>(
    mut maybe_changed: Query<(&mut Visibility, &VisibleIn<States>)>,
    state: Res<State<States>>,
) where
    States: bevy::prelude::States,
{
    for (mut vis, expected) in &mut maybe_changed {
        *vis = if &expected.0 == state.get() {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}
