use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::game::player::{Player, PlayerController, PlayerFacing};

pub struct GizmoHelpersPlugin;

impl Plugin for GizmoHelpersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, configure_gizmos)
            .add_systems(Update, (draw_world_gizmos, draw_player_gizmos));
    }
}

fn configure_gizmos(mut store: ResMut<GizmoConfigStore>) {
    let (config, _) = store.config_mut::<DefaultGizmoConfigGroup>();
    config.line.width = 1.5;
    config.depth_bias = 0.0;
}

fn draw_world_gizmos(mut gizmos: Gizmos) {
    // Axes at origin (drawn higher-level to stay visible).
    let axis_root = Vec3::new(0.0, 0.05, 0.0);
    let x_start = Vec3::new(0.0, 0.025, 0.0);
    let y_start = axis_root;
    let z_start = Vec3::new(0.0, 0.025, 0.0);
    let axis_len = 5.0;

    gizmos.line(
        x_start,
        x_start + Vec3::new(axis_len, 0.0, 0.0),
        Color::srgb(1.0, 0.1, 0.1),
    );
    gizmos.line(
        y_start,
        y_start + Vec3::new(0.0, axis_len, 0.0),
        Color::srgb(0.1, 1.0, 0.1),
    );
    gizmos.line(
        z_start,
        z_start + Vec3::new(0.0, 0.0, axis_len),
        Color::srgb(0.1, 0.4, 1.0),
    );

    // Simple ground grid for orientation
    let span = 30;
    for i in -span..=span {
        let p = i as f32 * 2.0;
        let color = if i % 5 == 0 {
            Color::srgb(0.35, 0.35, 0.38)
        } else {
            Color::srgb(0.18, 0.18, 0.2)
        };
        let height = 0.025;
        gizmos.line(
            Vec3::new(p, height, -span as f32 * 2.0),
            Vec3::new(p, height, span as f32 * 2.0),
            color,
        );
        gizmos.line(
            Vec3::new(-span as f32 * 2.0, height, p),
            Vec3::new(span as f32 * 2.0, height, p),
            color,
        );
    }
}

fn draw_player_gizmos(
    mut gizmos: Gizmos,
    player_query: Query<
        (
            &GlobalTransform,
            Option<&LinearVelocity>,
            Option<&PlayerFacing>,
            Option<&PlayerController>,
        ),
        With<Player>,
    >,
) {
    let Ok((player_tx, linear_velocity, facing, controller)) = player_query.single() else {
        return;
    };

    let pos = player_tx.translation();
    let radius = controller.map(|c| c.radius).unwrap_or(0.6);

    // Ground footprint and collider outline.
    gizmos.sphere(
        Isometry3d::from_translation(pos),
        radius,
        Color::srgb(0.32, 0.16, 0.6).with_alpha(0.1),
    );

    // Facing arrow and vision cone (90 deg).
    let yaw = facing.map(|f| f.yaw).unwrap_or(0.0);
    let facing_dir = Quat::from_rotation_y(yaw) * Vec3::NEG_Z;
    let vision_origin = pos + Vec3::Y * 0.1;
    let vision_len = 6.0;
    let half_angle = std::f32::consts::FRAC_PI_4;
    let left_dir = Quat::from_rotation_y(yaw + half_angle) * Vec3::NEG_Z;
    let right_dir = Quat::from_rotation_y(yaw - half_angle) * Vec3::NEG_Z;

    gizmos.arrow(
        vision_origin,
        vision_origin + facing_dir * vision_len,
        Color::srgb(1.0, 0.8, 0.2),
    );
    gizmos.arrow(
        vision_origin,
        vision_origin + left_dir * vision_len,
        Color::srgb(0.9, 0.6, 0.2),
    );
    gizmos.arrow(
        vision_origin,
        vision_origin + right_dir * vision_len,
        Color::srgb(0.9, 0.6, 0.2),
    );

    // Velocity vector (planar), clamped to the vision length.
    if let Some(vel) = linear_velocity {
        let planar = Vec3::new(vel.x, 0.0, vel.z);
        let planar = planar.clamp_length_max(vision_len);
        gizmos.arrow(
            pos + Vec3::Y * 0.05,
            pos + Vec3::Y * 0.05 + planar,
            Color::srgb(0.2, 0.9, 1.0),
        );
    }

    // Hearing radius.
    let hear_radius = 6.0;
    let hear_iso = Isometry3d::new(
        Vec3::new(pos.x, 0.1, pos.z),
        Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
    );
    gizmos.circle(
        hear_iso,
        hear_radius,
        Color::srgb(0.3, 0.8, 0.7).with_alpha(0.4),
    );
}
