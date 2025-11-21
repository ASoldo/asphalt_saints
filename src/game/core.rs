use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    InGame,
    Paused,
}

#[derive(Resource, Default, Debug)]
pub struct TimeScale(pub f32);

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_resource::<TimeScale>()
            .add_systems(Startup, bootstrap)
            .add_systems(OnEnter(GameState::Loading), finish_loading)
            .add_systems(OnEnter(GameState::Paused), pause_time)
            .add_systems(OnExit(GameState::Paused), resume_time);
    }
}

fn bootstrap(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::srgb(0.01, 0.01, 0.015)));
}

fn finish_loading(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::InGame);
}

fn pause_time(mut time: ResMut<Time<Virtual>>, mut time_scale: ResMut<TimeScale>) {
    time_scale.0 = time.relative_speed();
    time.set_relative_speed(0.0);
}

fn resume_time(mut time: ResMut<Time<Virtual>>, time_scale: Res<TimeScale>) {
    let speed = if time_scale.0 <= f32::EPSILON {
        1.0
    } else {
        time_scale.0
    };
    time.set_relative_speed(speed);
}
