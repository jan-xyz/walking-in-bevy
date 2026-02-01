use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    Forward,
    Backward,
    TurnLeft,
    TurnRight,
    Jump,
    SwapModel,
}

pub fn default_player_input_map() -> InputMap<PlayerAction> {
    InputMap::new([
        // Arrow-Keys
        (PlayerAction::Forward, KeyCode::ArrowUp),
        (PlayerAction::Backward, KeyCode::ArrowDown),
        (PlayerAction::TurnLeft, KeyCode::ArrowLeft),
        (PlayerAction::TurnRight, KeyCode::ArrowRight),
        // WASD
        (PlayerAction::Forward, KeyCode::KeyW),
        (PlayerAction::Backward, KeyCode::KeyS),
        (PlayerAction::TurnLeft, KeyCode::KeyA),
        (PlayerAction::TurnRight, KeyCode::KeyD),
        // Actions
        (PlayerAction::Jump, KeyCode::Space),
        (PlayerAction::SwapModel, KeyCode::Tab),
    ])
}
