use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Progression {
    pub cash: u32,
    pub skill_points: u32,
}

pub struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Progression>();
    }
}
