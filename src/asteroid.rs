use std::time::Duration;

use bevy::{color::palettes::css::RED, prelude::*, window::PrimaryWindow};
use rand::Rng;

pub struct AsteroidPlugin;

#[derive(Resource)]
struct AsteroidSpawnTimer {
    timer: Timer,
}

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AsteroidSpawnTimer {
            timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        });
        app.add_systems(Update, spawn_asteroid);
    }
}

fn spawn_asteroid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    mut spawn_timer: ResMut<AsteroidSpawnTimer>,
) {
    // Tick the timer
    spawn_timer.timer.tick(time.delta());

    // Only spawn if the timer finished
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::rng();
    let window = window_q.single();
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    // TODO: use this randomizer as the asteroid's target to set velocity direction
    let rand_x = rng.random_range(-half_width..half_width);
    let rand_y = rng.random_range(-half_height..half_height);

    // TODO: make a randomized starting position from outside the screen

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(25.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        Transform {
            translation: Vec3::new(rand_x, rand_y, 0.0),
            ..Default::default()
        },
    ));
}
