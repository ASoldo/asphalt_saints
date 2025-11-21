pub mod ai;
pub mod audio;
pub mod camera;
pub mod combat;
pub mod core;
pub mod debug;
pub mod faction;
pub mod input;
pub mod mission;
pub mod physics;
pub mod player;
pub mod progression;
pub mod ui;
pub mod vehicle;
pub mod world;

use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;

use crate::game::{
    ai::AiPlugin, audio::AudioPlugin, camera::CameraPlugin, combat::CombatPlugin, core::CorePlugin,
    debug::DebugPlugin, faction::FactionPlugin, input::InputPlugin, mission::MissionPlugin,
    physics::PhysicsPlugin, player::PlayerPlugin, progression::ProgressionPlugin, ui::UiPlugin,
    vehicle::VehiclePlugin, world::WorldPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .add_plugins(CorePlugin)
            .add_plugins(DebugPlugin)
            .add_plugins(PhysicsPlugin)
            .add_plugins(InputPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(WorldPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(CombatPlugin)
            .add_plugins(VehiclePlugin)
            .add_plugins(AiPlugin)
            .add_plugins(MissionPlugin)
            .add_plugins(FactionPlugin)
            .add_plugins(ProgressionPlugin)
            .add_plugins(AudioPlugin)
            .add_plugins(UiPlugin);
    }
}
