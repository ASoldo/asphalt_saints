use avian3d::prelude::*;
use bevy::prelude::*;

use crate::game::core::GameState;

#[derive(Resource, Clone)]
pub struct PhysicsConfig {
    pub gravity: Vec3,
    pub substeps: u32,
    pub length_unit: f32,
    pub enable_debug_render: bool,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0.0, -18.0, 0.0),
            substeps: 8,
            length_unit: 1.0,
            enable_debug_render: false,
        }
    }
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        let config = PhysicsConfig::default();
        let debug_enabled = config.enable_debug_render;
        let gravity = config.gravity;
        let substeps = config.substeps;
        let length_unit = config.length_unit;

        app.insert_resource(Gravity(gravity))
            .insert_resource(SubstepCount(substeps))
            .insert_resource(DefaultFriction(Friction::new(0.25)))
            .insert_resource(config);

        let plugins = PhysicsPlugins::default().with_length_unit(length_unit);
        app.add_plugins(plugins);

        if debug_enabled {
            app.add_plugins(PhysicsDebugPlugin::default());
        }

        app.add_systems(OnEnter(GameState::Paused), pause_physics);
        app.add_systems(OnExit(GameState::Paused), resume_physics);
    }
}

fn pause_physics(mut time: ResMut<Time<Physics>>) {
    time.pause();
}

fn resume_physics(mut time: ResMut<Time<Physics>>) {
    time.unpause();
}
