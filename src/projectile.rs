use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_projectiles);
    }
}

#[derive(Component)]
pub struct Projectile {
    pub speed: f32,
}

fn update_projectiles(
    mut commands: Commands,
    mut projectile_q: Query<(Entity, &mut Transform, &Projectile)>,
    window: Query<&Window>,
) {
    let window = window.single();
    let bound_x = window.width() / 2.0;
    let bound_y = window.height() / 2.0;

    for (entity, mut transform, projectile) in &mut projectile_q {
        // Move projectile in the direction it's facing
        let direction = transform.rotation * Vec3::X;
        transform.translation += direction * projectile.speed;

        // Despawn projectile_q that go off screen
        if transform.translation.x.abs() > bound_x || transform.translation.y.abs() > bound_y {
            commands.entity(entity).despawn();
        }
    }
}
