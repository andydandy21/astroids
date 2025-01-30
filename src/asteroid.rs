use std::time::Duration;

use bevy::{color::palettes::css::RED, prelude::*, window::PrimaryWindow};
use rand::{seq::IndexedRandom, Rng};

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
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::rng();
    let window = window_q.single();
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;
    let asteroid_radius = 25.0;

    // TODO: use this randomizer as the asteroid's target to set velocity direction
    let rand_x = rng.random_range(-half_width..half_width);
    let rand_y = rng.random_range(-half_height..half_height);

    // TODO: make a randomized starting position from outside the screen
    let spawn_points = vec!["top", "bottom", "left", "right"];
    let spawn_location = spawn_points.choose(&mut rng);
    let (spawn_x, spawn_y) = match spawn_location {
        Some(&"top") => (
            rng.random_range(-half_width..half_width),
            half_height + asteroid_radius,
        ),
        Some(&"bottom") => (
            rng.random_range(-half_width..half_width),
            -half_height - asteroid_radius,
        ),
        Some(&"left") => (
            -half_width - asteroid_radius,
            rng.random_range(-half_height..half_height),
        ),
        Some(&"right") => (
            half_width - asteroid_radius,
            rng.random_range(-half_height..half_height),
        ),
        _ => (0.0, 0.0),
    };

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(asteroid_radius))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        Transform {
            translation: Vec3::new(spawn_x, spawn_y, 0.0),
            ..Default::default()
        },
    ));
}
