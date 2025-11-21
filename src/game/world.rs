use avian3d::prelude::*;
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
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
            ground_height: 0.5,
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

fn spawn_world(
    mut commands: Commands,
    config: Res<WorldConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
        Mesh3d(
            meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(config.ground_size.x, config.ground_size.y),
            ),
        ),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.1, 0.11),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    commands.spawn((
        PointLight {
            intensity: 12_000.0,
            shadows_enabled: true,
            range: 200.0,
            ..default()
        },
        Transform::from_xyz(0.0, 12.0, 0.0),
    ));

    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(
            config.ground_size.x * 0.5,
            config.ground_height * 0.5,
            config.ground_size.y * 0.5,
        ),
        Friction::new(0.1),
        Transform::from_xyz(0.0, -config.ground_height * 0.5, 0.0),
        Name::new("Ground"),
    ));

    let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let cube_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.25, 0.35, 0.75),
        perceptual_roughness: 0.6,
        metallic: 0.05,
        ..default()
    });

    for i in 0..8 {
        let x = -6.0 + (i as f32 % 4.0) * 3.5;
        let z = -6.0 + (i as f32 / 4.0).floor() * 3.5;
        commands.spawn((
            RigidBody::Dynamic,
            Collider::cuboid(0.5, 0.5, 0.5),
            Mass(45.0),
            LinearDamping(0.0),
            AngularDamping(0.1),
            Mesh3d(cube_mesh.clone()),
            MeshMaterial3d(cube_mat.clone()),
            Transform::from_xyz(x, 2.0, z),
            Name::new(format!("Crate {i}")),
        ));
    }
}
