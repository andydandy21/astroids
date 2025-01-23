use bevy::{prelude::*, window::PrimaryWindow};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (move_player, track_mouse));
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
// TODO: change query to transform with player
fn move_player(
    mut player: Query<(&Player, &mut Transform)>,
    window: Query<&Window>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let window = window.single();
    let padding = 20.0;
    let speed = 250.0;

    let bound_x = window.width() / 2.0 - padding;
    let bound_y = window.height() / 2.0 - padding;

    let (setting, mut pos) = player.single_mut();
    if input.pressed(setting.move_up) {
        pos.translation.y = (pos.translation.y + speed * time.delta_secs()).min(bound_y);
    }
    if input.pressed(setting.move_down) {
        pos.translation.y = (pos.translation.y - speed * time.delta_secs()).max(-bound_y);
    }
    if input.pressed(setting.move_left) {
        pos.translation.x = (pos.translation.x - speed * time.delta_secs()).max(-bound_x);
    }
    if input.pressed(setting.move_right) {
        pos.translation.x = (pos.translation.x + speed * time.delta_secs()).min(bound_x);
    }
}

fn track_mouse(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut player_q: Query<&mut Transform, With<Player>>,
) {
    let window = window_q.single();
    if let Some(cursor_pos) = window.cursor_position() {
        let mut player_transform = player_q.single_mut();

        // Convert cursor position from screen coordinates to world coordinates
        let cursor_world_pos = Vec2::new(
            cursor_pos.x - window.width() / 2.0,
            -(cursor_pos.y - window.height() / 2.0), // Flip Y coordinate
        );

        // Get player position in 2D
        let player_pos = player_transform.translation.truncate();

        // Calculate the direction vector from player to cursor
        let direction = cursor_world_pos - player_pos;

        // Calculate the angle between the direction vector and the positive X-axis
        let angle = direction.y.atan2(direction.x);

        // Set the rotation of the player
        player_transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2);
    }
}
