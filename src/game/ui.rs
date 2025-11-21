use bevy::prelude::*;
use bevy::ui::{PositionType, UiRect, Val};

use crate::game::combat::Health;
use crate::game::core::GameState;
use crate::game::progression::Progression;

#[derive(Component)]
struct HudRoot;

#[derive(Component)]
struct HudLine;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_hud)
            .add_systems(Update, update_hud.run_if(in_state(GameState::InGame)));
    }
}

fn spawn_hud(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(16.0)),
                ..default()
            },
            HudRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(12.0),
                    left: Val::Px(12.0),
                    ..default()
                },
                Text::new("HUD initializing..."),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                HudLine,
            ));
        });
}

fn update_hud(
    player_health: Query<&Health>,
    progression: Res<Progression>,
    mut query: Query<&mut Text, With<HudLine>>,
) {
    let Some(mut text) = query.iter_mut().next() else {
        return;
    };

    let health = player_health.iter().next().map(|h| (h.current, h.max));
    let health_line = match health {
        Some((current, max)) => format!("HP: {:.0}/{:.0}", current, max),
        None => "HP: --".to_string(),
    };

    *text = Text::new(format!(
        "{health_line}   Cash: ${}   Skill: {}",
        progression.cash, progression.skill_points
    ));
}
