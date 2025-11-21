use avian3d::prelude::*;
use bevy::prelude::*;

use crate::game::combat::Health;
use crate::game::core::GameState;
use crate::game::input::PlayerInput;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerPawn;

#[derive(Component)]
pub struct PlayerController {
    pub walk_speed: f32,
    pub sprint_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            walk_speed: 9.5,
            sprint_speed: 13.0,
            acceleration: 60.0,
            deceleration: 30.0,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_player)
            .add_systems(Update, drive_player.run_if(in_state(GameState::InGame)));
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        PlayerPawn,
        PlayerController::default(),
        RigidBody::Dynamic,
        Collider::capsule(0.6, 0.9),
        LockedAxes::new().lock_rotation_x().lock_rotation_z(),
        Mass(110.0),
        LinearDamping(2.5),
        AngularDamping(6.0),
        MaxLinearSpeed(25.0),
        Transform::from_xyz(0.0, 2.5, 0.0).with_rotation(Quat::from_rotation_y(0.0)),
        Health::new(150.0),
        Name::new("Player"),
    ));
}

fn drive_player(
    time: Res<Time>,
    input: Res<PlayerInput>,
    mut query: Query<(&PlayerController, &mut LinearVelocity, &mut Transform), With<Player>>,
) {
    let Some((config, mut velocity, mut transform)) = query.iter_mut().next() else {
        return;
    };

    // Invert forward so W moves toward -Z (away from camera) in world space.
    let desired_direction = Vec3::new(input.movement.x, 0.0, -input.movement.y);
    let max_speed = if input.sprint {
        config.sprint_speed
    } else {
        config.walk_speed
    };

    let target_velocity = desired_direction * max_speed;
    let current_velocity = velocity.0;
    let current_vertical = velocity.y;

    let delta = time.delta_secs();
    let accel_rate = if target_velocity.length_squared() > 0.0 {
        config.acceleration
    } else {
        config.deceleration
    };

    let blended = current_velocity.lerp(target_velocity, (accel_rate * delta).clamp(0.0, 1.0));
    velocity.0 = Vec3::new(blended.x, current_vertical, blended.z);

    if desired_direction.length_squared() > 0.25 {
        let facing = Vec3::new(desired_direction.x, 0.0, desired_direction.z).normalize();
        transform.rotation = transform.rotation.slerp(Quat::from_rotation_y(facing.x.atan2(facing.z)), 0.35);
    }
}
