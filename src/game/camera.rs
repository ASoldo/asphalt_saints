use bevy::prelude::*;

use crate::game::core::GameState;
use crate::game::input::PlayerInput;
use crate::game::player::Player;

#[derive(Resource)]
pub struct TopDownCameraConfig {
    pub height: f32,
    pub distance: f32,
    pub pitch_radians: f32,
    pub damping: f32,
    pub zoom_step: f32,
    pub min_height: f32,
    pub max_height: f32,
}

impl Default for TopDownCameraConfig {
    fn default() -> Self {
        Self {
            height: 16.0,
            distance: 18.0,
            pitch_radians: 55.0_f32.to_radians(),
            damping: 8.0,
            zoom_step: 2.0,
            min_height: 8.0,
            max_height: 40.0,
        }
    }
}

#[derive(Component)]
pub struct TopDownCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TopDownCameraConfig>()
            .add_systems(OnEnter(GameState::InGame), spawn_camera)
            .add_systems(Update, (track_player, handle_zoom).run_if(in_state(GameState::InGame)));
    }
}

fn spawn_camera(mut commands: Commands, config: Res<TopDownCameraConfig>) {
    let focus_translation = Vec3::ZERO;
    let rotated_offset =
        Quat::from_rotation_x(-config.pitch_radians) * Vec3::new(0.0, 0.0, config.distance);
    let eye = focus_translation + rotated_offset + Vec3::Y * config.height;

    commands.spawn((
        TopDownCamera,
        Camera3d::default(),
        Transform::from_translation(eye).looking_at(focus_translation, Vec3::Y),
    ));
}

fn track_player(
    time: Res<Time>,
    config: Res<TopDownCameraConfig>,
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<TopDownCamera>, Without<Player>)>,
) {
    let Some(player_transform) = player_query.iter().next() else {
        return;
    };

    let Some(mut camera_transform) = camera_query.iter_mut().next() else {
        return;
    };

    let rotated_offset =
        Quat::from_rotation_x(-config.pitch_radians) * Vec3::new(0.0, 0.0, config.distance);
    let target_translation = player_transform.translation + rotated_offset + Vec3::Y * config.height;
    let dt = time.delta_secs();
    camera_transform.translation = camera_transform
        .translation
        .lerp(target_translation, (config.damping * dt).clamp(0.0, 1.0));
    camera_transform.look_at(player_transform.translation, Vec3::Y);
}

fn handle_zoom(
    player_input: Res<PlayerInput>,
    mut config: ResMut<TopDownCameraConfig>,
    mut camera_query: Query<&mut Transform, With<TopDownCamera>>,
) {
    let zoom_delta = player_input.camera_zoom;
    if zoom_delta.abs() < f32::EPSILON {
        return;
    }

    config.height = (config.height + zoom_delta * config.zoom_step).clamp(config.min_height, config.max_height);
    config.distance = (config.distance + zoom_delta * config.zoom_step).clamp(config.min_height, config.max_height + 4.0);

    if let Some(mut transform) = camera_query.iter_mut().next() {
        transform.translation.y = config.height;
    }
}
