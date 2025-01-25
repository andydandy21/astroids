use bevy::{color::palettes::css::BLUE, prelude::*, window::PrimaryWindow};

use crate::projectile::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProjectilePlugin);
        app.add_systems(Startup, spawn_player);
        app.add_systems(FixedUpdate, move_player);
        app.add_systems(Update, (track_mouse, shoot_projectile));
    }
}

#[derive(Component)]
struct Player {
    move_up: KeyCode,
    move_down: KeyCode,
    move_left: KeyCode,
    move_right: KeyCode,
    projectile_timer: Timer,
}

fn spawn_player(mut commands: Commands) {
    let fire_rate = 5.0; // shots per second

    commands.spawn((
        Sprite::from_color(Color::WHITE, Vec2::new(25., 25.)),
        Player {
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            projectile_timer: Timer::from_seconds(1.0 / fire_rate, TimerMode::Repeating),
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
    ));
}

fn move_player(
    mut player: Query<(&Player, &mut Transform)>,
    window: Query<&Window>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let window = window.single();
    let padding = 20.0;
    let speed = 6.0;

    let bound_x = window.width() / 2.0 - padding;
    let bound_y = window.height() / 2.0 - padding;

    let (setting, mut pos) = player.single_mut();
    if input.pressed(setting.move_up) {
        pos.translation.y = (pos.translation.y + speed).min(bound_y);
    }
    if input.pressed(setting.move_down) {
        pos.translation.y = (pos.translation.y - speed).max(-bound_y);
    }
    if input.pressed(setting.move_left) {
        pos.translation.x = (pos.translation.x - speed).max(-bound_x);
    }
    if input.pressed(setting.move_right) {
        pos.translation.x = (pos.translation.x + speed).min(bound_x);
    }
}

fn track_mouse(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut player_q: Query<&mut Transform, With<Player>>,
) {
    let window = window_q.single();
    if let Some(cursor_pos) = window.cursor_position() {
        let mut player_transform = player_q.single_mut();
        let cursor_world_pos = Vec2::new(
            cursor_pos.x - window.width() / 2.0,
            -(cursor_pos.y - window.height() / 2.0), // Flip Y coordinate
        );
        let player_pos = player_transform.translation.truncate();
        let direction = cursor_world_pos - player_pos;
        let angle = direction.y.atan2(direction.x);
        player_transform.rotation = Quat::from_rotation_z(angle);
    }
}

fn shoot_projectile(
    mut commands: Commands,
    input: Res<ButtonInput<MouseButton>>,
    mut player_q: Query<(&Transform, &mut Player)>,
    time: Res<Time>,
) {
    let (player_pos, mut player) = player_q.single_mut();
    player.projectile_timer.tick(time.delta());

    if input.pressed(MouseButton::Left) && player.projectile_timer.just_finished() {
        commands.spawn((
            Sprite::from_color(BLUE, Vec2::new(20.0, 5.0)),
            Transform {
                translation: Vec3::new(player_pos.translation.x, player_pos.translation.y, 0.0),
                rotation: player_pos.rotation,
                ..Default::default()
            },
            Projectile { speed: 25.0 },
        ));
    }
}
