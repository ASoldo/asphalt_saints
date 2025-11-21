use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::game::{
    camera::TopDownCamera,
    player::{Player, PlayerController, PlayerFacing},
};

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
    config.depth_bias = -0.005;
}

fn draw_world_gizmos(mut gizmos: Gizmos) {
    // Axes at origin
    gizmos.arrow(Vec3::ZERO, Vec3::X * 5.0, Color::srgb(1.0, 0.1, 0.1));
    gizmos.arrow(Vec3::ZERO, Vec3::Y * 5.0, Color::srgb(0.1, 1.0, 0.1));
    gizmos.arrow(Vec3::ZERO, Vec3::Z * 5.0, Color::srgb(0.1, 0.4, 1.0));

    // Simple ground grid for orientation
    let span = 30;
    for i in -span..=span {
        let p = i as f32 * 2.0;
        let color = if i % 5 == 0 {
            Color::srgb(0.35, 0.35, 0.38)
        } else {
            Color::srgb(0.18, 0.18, 0.2)
        };
        let height = 0.05;
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
    camera_query: Query<&GlobalTransform, With<TopDownCamera>>,
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

    // Facing arrow.
    let yaw = facing.map(|f| f.yaw).unwrap_or(0.0);
    let facing_dir = Quat::from_rotation_y(yaw) * Vec3::NEG_Z;
    gizmos.arrow(
        pos + Vec3::Y * 0.1,
        pos + Vec3::Y * 0.1 + facing_dir * 2.0,
        Color::srgb(1.0, 0.8, 0.2),
    );

    // Velocity vector (planar).
    if let Some(vel) = linear_velocity {
        let planar = Vec3::new(vel.x, 0.0, vel.z);
        gizmos.arrow(
            pos + Vec3::Y * 0.05,
            pos + Vec3::Y * 0.05 + planar,
            Color::srgb(0.2, 0.9, 1.0),
        );
    }

}
