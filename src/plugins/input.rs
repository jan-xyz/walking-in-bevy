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

pub fn default_player1_input_map() -> InputMap<PlayerAction> {
    InputMap::new([
        // Movement
        (PlayerAction::Forward, KeyCode::KeyW),
        (PlayerAction::Backward, KeyCode::KeyS),
        (PlayerAction::TurnLeft, KeyCode::KeyA),
        (PlayerAction::TurnRight, KeyCode::KeyD),
        // Actions
        (PlayerAction::Jump, KeyCode::ShiftLeft),
        (PlayerAction::SwapModel, KeyCode::Tab),
    ])
}

pub fn default_player2_input_map() -> InputMap<PlayerAction> {
    InputMap::new([
        // Movement
        (PlayerAction::Forward, KeyCode::ArrowUp),
        (PlayerAction::Backward, KeyCode::ArrowDown),
        (PlayerAction::TurnLeft, KeyCode::ArrowLeft),
        (PlayerAction::TurnRight, KeyCode::ArrowRight),
        // Actions
        (PlayerAction::Jump, KeyCode::ShiftRight),
        (PlayerAction::SwapModel, KeyCode::Slash),
    ])
}
