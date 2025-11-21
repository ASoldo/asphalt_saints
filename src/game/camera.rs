use bevy::prelude::*;

use crate::game::core::GameState;
use crate::game::input::PlayerInput;
use crate::game::player::Player;

#[derive(Resource)]
pub struct TopDownCameraConfig {
    pub height: f32,
    pub pitch_radians: f32,
    pub damping: f32,
    pub zoom_step: f32,
    pub min_height: f32,
    pub max_height: f32,
}

impl Default for TopDownCameraConfig {
    fn default() -> Self {
        Self {
            height: 22.0,
            pitch_radians: 90.0_f32.to_radians(),
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
            .add_systems(
                PostUpdate,
                (ensure_camera, update_camera_pose, handle_zoom)
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

fn camera_offset(config: &TopDownCameraConfig) -> Vec3 {
    Vec3::Y * config.height
}

fn ensure_camera(
    mut commands: Commands,
    config: Res<TopDownCameraConfig>,
    player_query: Query<&GlobalTransform, With<Player>>,
    camera_query: Query<Entity, With<TopDownCamera>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    if camera_query.iter().next().is_some() {
        return;
    }

    let start = player_transform.translation() + camera_offset(&config);
    commands.spawn((
        TopDownCamera,
        Camera3d::default(),
        Transform::from_translation(start)
            .with_rotation(Quat::from_rotation_x(-config.pitch_radians)),
    ));
}

fn update_camera_pose(
    time: Res<Time>,
    config: Res<TopDownCameraConfig>,
    player_query: Query<&GlobalTransform, With<Player>>,
    mut camera_query: Query<&mut Transform, With<TopDownCamera>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let Ok(mut cam_transform) = camera_query.single_mut() else {
        return;
    };

    let player_pos = player_transform.translation();
    let target_pos = player_pos + camera_offset(&config);
    let lerp_alpha = 1.0 - (-config.damping * time.delta_secs()).exp();

    cam_transform.translation = cam_transform
        .translation
        .lerp(target_pos, lerp_alpha.clamp(0.0, 1.0));
    cam_transform.rotation = Quat::from_rotation_x(-config.pitch_radians);
}

fn handle_zoom(player_input: Res<PlayerInput>, mut config: ResMut<TopDownCameraConfig>) {
    let zoom_delta = player_input.camera_zoom;
    if zoom_delta.abs() < f32::EPSILON {
        return;
    }

    config.height =
        (config.height + zoom_delta * config.zoom_step).clamp(config.min_height, config.max_height);
}
