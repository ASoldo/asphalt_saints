use bevy::prelude::*;

use crate::game::core::GameState;
use crate::game::input::PlayerInput;
use crate::game::player::Player;

#[derive(Resource)]
pub struct TopDownCameraConfig {
    pub height: f32,
    pub distance: f32,
    pub pitch_radians: f32,
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
                (ensure_camera, handle_zoom)
                    .run_if(in_state(GameState::InGame))
                    .after(TransformSystems::Propagate),
            );
    }
}

fn camera_offset(config: &TopDownCameraConfig) -> Vec3 {
    Vec3::Y * config.height
}

fn camera_transform_world(player_translation: Vec3, config: &TopDownCameraConfig) -> Transform {
    Transform {
        translation: player_translation + camera_offset(config),
        rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
        ..default()
    }
}

fn ensure_camera(
    mut commands: Commands,
    config: Res<TopDownCameraConfig>,
    player_query: Query<Entity, With<Player>>,
    camera_query: Query<Entity, With<TopDownCamera>>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };

    if camera_query.iter().next().is_some() {
        return;
    }

    commands.entity(player_entity).with_children(|parent| {
        parent.spawn((
            TopDownCamera,
            Camera3d::default(),
            Transform {
                translation: camera_offset(&config),
                rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
                ..default()
            },
        ));
    });
}

fn handle_zoom(
    player_input: Res<PlayerInput>,
    mut config: ResMut<TopDownCameraConfig>,
) {
    let zoom_delta = player_input.camera_zoom;
    if zoom_delta.abs() < f32::EPSILON {
        return;
    }

    config.height =
        (config.height + zoom_delta * config.zoom_step).clamp(config.min_height, config.max_height);
}
