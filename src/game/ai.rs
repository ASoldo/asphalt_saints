use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct FactionBrain {
    pub alert: bool,
}

#[derive(Component, Debug)]
pub enum AiRole {
    Pedestrian,
    Cop,
    GangSoldier,
    Boss,
}

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick_ai);
    }
}

fn tick_ai(mut query: Query<(&AiRole, &mut FactionBrain)>) {
    for (_role, mut brain) in &mut query {
        brain.alert = brain.alert || false;
    }
}
