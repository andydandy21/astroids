use bevy::prelude::*;

mod player;
use player::PlayerPlugin;

mod projectile;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            fullsize_content_view: true,
            ..Default::default()
        }),
        ..Default::default()
    }));
    app.add_systems(Startup, spawn_camera);
    app.add_plugins(PlayerPlugin);
    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
