use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Vehicle {
    pub name: &'static str,
    pub max_speed: f32,
    pub acceleration: f32,
}

#[derive(Component, Default)]
pub struct VehicleInput {
    pub throttle: f32,
    pub steer: f32,
    pub handbrake: bool,
}

pub struct VehiclePlugin;

impl Plugin for VehiclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_vehicle_input);
    }
}

fn apply_vehicle_input(
    mut query: Query<(&Vehicle, &mut LinearVelocity, &VehicleInput)>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();
    for (vehicle, mut velocity, input) in &mut query {
        let target_speed = vehicle.max_speed * input.throttle.clamp(-1.0, 1.0);
        let blended = velocity
            .z
            .lerp(target_speed, (vehicle.acceleration * delta).clamp(0.0, 1.0));
        velocity.z = blended;

        if input.handbrake {
            velocity.z *= 0.9_f32.powf(delta * 60.0);
        }

        velocity.x = input.steer.clamp(-1.0, 1.0) * (vehicle.max_speed * 0.1);
    }
}
