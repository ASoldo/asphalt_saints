use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

#[derive(Resource, Default, Debug, Clone, Copy)]
pub struct PlayerInput {
    pub movement: Vec2,
    pub look_delta: Vec2,
    pub yaw_input: f32,
    pub fire_primary: bool,
    pub fire_secondary: bool,
    pub sprint: bool,
    pub interact: bool,
    pub jump: bool,
    pub camera_zoom: f32,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerInput>()
            .add_systems(Update, gather_player_input);
    }
}

fn gather_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: MessageReader<MouseMotion>,
    mut player_input: ResMut<PlayerInput>,
) {
    let forward =
        keyboard.pressed(KeyCode::KeyW) as i8 as f32 - keyboard.pressed(KeyCode::KeyS) as i8 as f32;
    let forward_alt = keyboard.pressed(KeyCode::Space) as i8 as f32
        - keyboard.pressed(KeyCode::Backspace) as i8 as f32;
    let strafe =
        keyboard.pressed(KeyCode::KeyD) as i8 as f32 - keyboard.pressed(KeyCode::KeyA) as i8 as f32;
    let movement = Vec2::new(strafe, forward + forward_alt);

    let mut look_delta = Vec2::ZERO;
    for motion in mouse_motion_events.read() {
        look_delta += motion.delta;
    }

    let camera_zoom = if keyboard.just_pressed(KeyCode::Equal) {
        -1.0
    } else if keyboard.just_pressed(KeyCode::Minus) {
        1.0
    } else {
        0.0
    };

    *player_input = PlayerInput {
        movement: if movement.length_squared() > 1.0 {
            movement.normalize()
        } else {
            movement
        },
        look_delta,
        yaw_input: keyboard.pressed(KeyCode::ArrowLeft) as i8 as f32
            - keyboard.pressed(KeyCode::ArrowRight) as i8 as f32,
        fire_primary: mouse_buttons.pressed(MouseButton::Left),
        fire_secondary: mouse_buttons.pressed(MouseButton::Right),
        sprint: keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight),
        interact: keyboard.just_pressed(KeyCode::KeyE),
        jump: keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::NumpadEnter),
        camera_zoom,
    };
}
