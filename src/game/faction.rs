use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FactionId {
    RustbornChoir,
    VelvetAlgorithm,
    StreetOracles,
    NeonJackals,
    TideSyndicate,
}

#[derive(Resource, Default)]
pub struct FactionRespect {
    pub respect: [i32; 5],
}

pub struct FactionPlugin;

impl Plugin for FactionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FactionRespect>();
    }
}
