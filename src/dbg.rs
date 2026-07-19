use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(add_debug_helper);
}

#[cfg(debug_assertions)]
pub fn add_debug_helper(app: &mut App) {
    use bevy::input::common_conditions::input_toggle_active;
    use bevy::prelude::KeyCode;
    use bevy_inspector_egui::bevy_egui::EguiPlugin;
    use bevy_inspector_egui::quick::WorldInspectorPlugin;

    app.add_plugins(EguiPlugin::default()).add_plugins(
        WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Backquote)),
    );
}

#[cfg(not(debug_assertions))]
pub fn add_debug_helper(_app: &mut App) {}
