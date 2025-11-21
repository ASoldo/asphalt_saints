use bevy::prelude::*;

use crate::game::core::GameState;
use crate::game::faction::FactionId;

#[derive(Debug, Clone)]
pub struct MissionDefinition {
    pub id: &'static str,
    pub faction: FactionId,
    pub title: &'static str,
    pub brief: &'static str,
}

#[derive(Resource, Default)]
pub struct MissionLog {
    pub active: Option<MissionDefinition>,
    pub completed: Vec<&'static str>,
}

pub struct MissionPlugin;

impl Plugin for MissionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MissionLog>()
            .add_systems(OnEnter(GameState::InGame), select_intro_mission);
    }
}

fn select_intro_mission(mut mission_log: ResMut<MissionLog>) {
    if mission_log.active.is_none() {
        mission_log.active = Some(MissionDefinition {
            id: "street_scramble",
            faction: FactionId::RustbornChoir,
            title: "Street Scramble",
            brief: "Cause havoc to announce your arrival in Neon Parish.",
        });
    }
}
