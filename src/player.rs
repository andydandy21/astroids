use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, move_player);
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
