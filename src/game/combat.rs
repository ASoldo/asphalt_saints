use bevy::prelude::*;

#[derive(Message, Debug, Clone)]
pub struct DamageEvent {
    pub entity: Entity,
    pub amount: f32,
    pub source: Option<Entity>,
}

#[derive(Component, Debug, Clone)]
pub struct Health {
    pub current: f32,
    pub max: f32,
    pub invulnerable: bool,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
            invulnerable: false,
        }
    }

    pub fn apply(&mut self, amount: f32) {
        if self.invulnerable {
            return;
        }
        self.current = (self.current - amount).clamp(0.0, self.max);
    }
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DamageEvent>()
            .add_systems(Update, (apply_damage, prune_dead));
    }
}

fn apply_damage(mut events: MessageReader<DamageEvent>, mut query: Query<&mut Health>) {
    for event in events.read() {
        if let Ok(mut health) = query.get_mut(event.entity) {
            health.apply(event.amount);
        }
    }
}

fn prune_dead(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in &query {
        if health.current <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
