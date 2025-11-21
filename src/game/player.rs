use avian3d::prelude::*;
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::*;

use crate::game::combat::Health;
use crate::game::core::GameState;
use crate::game::input::PlayerInput;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerPawn;

#[derive(Component)]
pub struct PlayerVisual;

#[derive(Component, Clone, Copy)]
pub struct PlayerController {
    pub walk_speed: f32,
    pub sprint_speed: f32,
    pub acceleration: f32,
    pub braking: f32,
    pub yaw_lerp_speed: f32,
    pub turn_speed: f32,
    pub lateral_damping: f32,
    pub drag: f32,
    pub radius: f32,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            walk_speed: 7.5,
            sprint_speed: 12.5,
            acceleration: 30.0,
            braking: 36.0,
            yaw_lerp_speed: 12.0,
            turn_speed: 5.6,
            lateral_damping: 10.0,
            drag: 1.5,
            radius: 0.6,
        }
    }
}

#[derive(Component, Debug)]
pub struct PlayerFacing {
    pub yaw: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_player)
            .add_systems(Update, drive_player.run_if(in_state(GameState::InGame)));
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let controller = PlayerController::default();

    commands
        .spawn((
            Player,
            PlayerPawn,
            controller,
            RigidBody::Dynamic,
            Collider::sphere(controller.radius),
            Friction::new(0.6),
            Mass(110.0),
            LinearDamping(0.3),
            AngularDamping(0.8),
            LockedAxes::new().lock_rotation_x().lock_rotation_z(),
            MaxLinearSpeed(25.0),
            Transform::from_xyz(0.0, 1.2, 0.0).with_rotation(Quat::from_rotation_y(0.0)),
            Health::new(150.0),
            PlayerFacing { yaw: 0.0 },
            Name::new("Player"),
        ))
        .with_children(|parent| {
            parent.spawn((
                Mesh3d(meshes.add(Sphere::new(controller.radius).mesh().uv(24, 16))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.7, 0.18, 0.3),
                    metallic: 0.15,
                    perceptual_roughness: 0.5,
                    ..default()
                })),
                Transform::from_xyz(0.0, -0.8, 0.0),
                PlayerVisual,
            ));
        });
}

fn drive_player(
    time: Res<Time>,
    input: Res<PlayerInput>,
    mut player_query: Query<
        (
            &PlayerController,
            &mut LinearVelocity,
            &mut PlayerFacing,
            &Children,
        ),
        With<Player>,
    >,
    mut visuals: Query<&mut Transform, With<PlayerVisual>>,
) {
    let Some((config, mut velocity, mut facing, children)) = player_query.iter_mut().next() else {
        return;
    };

    // GTA2-style: left/right steer, forward/back throttle.
    let turn_input = (input.movement.x + input.yaw_input).clamp(-1.0, 1.0);
    let turn_rate = turn_input * config.turn_speed;
    if turn_rate.abs() > 0.001 {
        facing.yaw = (facing.yaw + turn_rate * time.delta_secs()).rem_euclid(std::f32::consts::TAU);
    }

    let delta = time.delta_secs();
    let forward_input = input.movement.y.clamp(-1.0, 1.0);
    let forward_dir = Quat::from_rotation_y(facing.yaw) * Vec3::NEG_Z;
    let max_speed = if input.sprint {
        config.sprint_speed
    } else {
        config.walk_speed
    };

    let mut planar = Vec3::new(velocity.x, 0.0, velocity.z);
    // Thrust forward/backward.
    planar += forward_dir * (forward_input * config.acceleration * delta);

    // Brake when no input.
    if forward_input.abs() < 0.01 {
        let speed = planar.length();
        let new_speed = (speed - config.braking * delta).max(0.0);
        planar = if speed > 0.0 {
            planar.normalize() * new_speed
        } else {
            planar
        };
    }

    // Bleed sideways drift to keep a planted feel.
    let lateral = planar - forward_dir * planar.dot(forward_dir);
    planar -= lateral * (1.0 - (-config.lateral_damping * delta).exp());

    // Apply mild drag to prevent runaway speed.
    planar *= (1.0 - config.drag * delta).max(0.0);

    // Clamp to the allowed top speed.
    if planar.length_squared() > max_speed * max_speed {
        planar = planar.normalize() * max_speed;
    }

    velocity.0 = Vec3::new(planar.x, velocity.y, planar.z);

    if forward_input.abs() > 0.05 || turn_input.abs() > 0.05 {
        let yaw = facing.yaw;
        let yaw_alpha = 1.0 - (-config.yaw_lerp_speed * delta).exp();
        for child in children {
            if let Ok(mut t) = visuals.get_mut(*child) {
                let target_rot = Quat::from_rotation_y(yaw);
                t.rotation = t.rotation.slerp(target_rot, yaw_alpha.clamp(0.0, 1.0));
            }
        }
    }
}
