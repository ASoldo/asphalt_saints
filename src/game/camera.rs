use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::game::core::GameState;
use crate::game::input::PlayerInput;
use crate::game::player::{Player, PlayerFacing};

#[derive(Resource)]
pub struct TopDownCameraConfig {
    pub height: f32,
    pub distance: f32,
    pub pitch_radians: f32,
    pub damping: f32,
    pub zoom_step: f32,
    pub min_height: f32,
    pub max_height: f32,
    pub velocity_lead: f32,
    pub heading_lead: f32,
    pub lead_responsiveness: f32,
}

impl Default for TopDownCameraConfig {
    fn default() -> Self {
        Self {
            height: 16.0,
            distance: 18.0,
            pitch_radians: 55.0_f32.to_radians(),
            damping: 6.0,
            zoom_step: 2.0,
            min_height: 8.0,
            max_height: 40.0,
            velocity_lead: 0.25,
            heading_lead: 0.6,
            lead_responsiveness: 6.0,
        }
    }
}

#[derive(Component)]
pub struct TopDownCamera;

#[derive(Component, Default)]
pub struct CameraLead {
    pub offset: Vec3,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TopDownCameraConfig>()
            .add_systems(OnEnter(GameState::InGame), spawn_camera)
            .add_systems(
                PostUpdate,
                (track_player, handle_zoom)
                    .run_if(in_state(GameState::InGame))
                    .after(TransformSystems::Propagate),
            );
    }
}

fn spawn_camera(mut commands: Commands, config: Res<TopDownCameraConfig>) {
    let focus_translation = Vec3::ZERO;
    let rotated_offset =
        Quat::from_rotation_x(-config.pitch_radians) * Vec3::new(0.0, 0.0, config.distance);
    let eye = focus_translation + rotated_offset + Vec3::Y * config.height;

    commands.spawn((
        TopDownCamera,
        CameraLead::default(),
        Camera3d::default(),
        Transform::from_translation(eye).looking_at(focus_translation, Vec3::Y),
    ));
}

fn track_player(
    time: Res<Time>,
    config: Res<TopDownCameraConfig>,
    player_query: Query<
        (
            &GlobalTransform,
            Option<&LinearVelocity>,
            Option<&PlayerFacing>,
        ),
        With<Player>,
    >,
    mut camera_query: Query<
        (&mut Transform, &mut CameraLead),
        (With<TopDownCamera>, Without<Player>),
    >,
) {
    let Some((player_transform, linear_velocity, facing)) = player_query.iter().next() else {
        return;
    };

    let Some((mut camera_transform, mut lead)) = camera_query.iter_mut().next() else {
        return;
    };

    let planar_velocity = linear_velocity
        .map(|v| Vec3::new(v.x, 0.0, v.z))
        .unwrap_or(Vec3::ZERO);
    let dt = time.delta_secs();
    let speed = planar_velocity.length();
    let heading_dir = facing
        .map(|f| Quat::from_rotation_y(f.yaw) * Vec3::NEG_Z)
        .unwrap_or(Vec3::NEG_Z);

    let rotated_offset =
        Quat::from_rotation_x(-config.pitch_radians) * Vec3::new(0.0, 0.0, config.distance);
    let desired_lead =
        planar_velocity * config.velocity_lead + heading_dir * (speed * config.heading_lead);
    let lead_alpha = 1.0 - (-config.lead_responsiveness * dt).exp();
    lead.offset = lead.offset.lerp(desired_lead, lead_alpha.clamp(0.0, 1.0));

    let focus = player_transform.translation() + lead.offset;
    let target_translation = focus + rotated_offset + Vec3::Y * config.height;
    let lerp_alpha = 1.0 - (-config.damping * dt).exp();
    camera_transform.translation = camera_transform
        .translation
        .lerp(target_translation, lerp_alpha.clamp(0.0, 1.0));
    camera_transform.look_at(focus, Vec3::Y);
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

    config.height =
        (config.height + zoom_delta * config.zoom_step).clamp(config.min_height, config.max_height);
    config.distance = (config.distance + zoom_delta * config.zoom_step)
        .clamp(config.min_height, config.max_height + 4.0);

    if let Some(mut transform) = camera_query.iter_mut().next() {
        transform.translation.y = config.height;
    }
}
