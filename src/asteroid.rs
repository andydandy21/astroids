use bevy::{color::palettes::css::RED, prelude::*, window::PrimaryWindow};
use rand::Rng;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_asteroid);
    }
}

fn spawn_asteroid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let mut rng = rand::rng();
    let window = window_q.single();
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    let rand_x = rng.random_range(-half_width..half_width);
    let rand_y = rng.random_range(-half_height..half_height);

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(25.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        Transform {
            translation: Vec3::new(rand_x, rand_y, 0.0),
            ..Default::default()
        },
    ));
}
