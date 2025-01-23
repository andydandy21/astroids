use bevy::{prelude::*, window::PrimaryWindow};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(FixedUpdate, move_player);
        app.add_systems(Update, track_mouse);
    }
}

#[derive(Component)]
struct Player {
    move_up: KeyCode,
    move_down: KeyCode,
    move_left: KeyCode,
    move_right: KeyCode,
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(Color::WHITE, Vec2::new(25., 25.)),
        Player {
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
        },
    ));
}

// TODO: Check the FixedUpdate schedule and see if it works better than using Time
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
