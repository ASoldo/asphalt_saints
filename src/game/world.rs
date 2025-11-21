use avian3d::prelude::*;
use bevy::prelude::*;

use crate::game::core::GameState;

#[derive(Resource)]
pub struct WorldConfig {
    pub ground_size: Vec2,
    pub ground_height: f32,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            ground_size: Vec2::new(512.0, 512.0),
            ground_height: 1.0,
        }
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldConfig>()
            .add_systems(OnEnter(GameState::InGame), spawn_world);
    }
}

fn spawn_world(mut commands: Commands, config: Res<WorldConfig>) {
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.45, 0.48, 0.55),
        brightness: 300.0,
        affects_lightmapped_meshes: true,
    });

    commands.spawn((
        DirectionalLight {
            illuminance: 50_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-15.0, 22.0, -8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(config.ground_size.x, config.ground_height, config.ground_size.y),
        Transform::from_xyz(0.0, -config.ground_height, 0.0),
        Name::new("Ground"),
    ));
}
