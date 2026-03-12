use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use serde::{Deserialize, Serialize};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerActions>::default());
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect, Serialize, Deserialize)]
pub enum PlayerActions {
    Forward,
    Backward,
    TurnLeft,
    TurnRight,
    Jump,
    SwapModel,
}

pub fn default_player1_input_map() -> InputMap<PlayerActions> {
    InputMap::new([
        // Movement
        (PlayerActions::Forward, KeyCode::KeyW),
        (PlayerActions::Backward, KeyCode::KeyS),
        (PlayerActions::TurnLeft, KeyCode::KeyA),
        (PlayerActions::TurnRight, KeyCode::KeyD),
        // Actions
        (PlayerActions::Jump, KeyCode::ShiftLeft),
        (PlayerActions::SwapModel, KeyCode::Tab),
    ])
}

pub fn default_player2_input_map() -> InputMap<PlayerActions> {
    InputMap::new([
        // Movement
        (PlayerActions::Forward, KeyCode::ArrowUp),
        (PlayerActions::Backward, KeyCode::ArrowDown),
        (PlayerActions::TurnLeft, KeyCode::ArrowLeft),
        (PlayerActions::TurnRight, KeyCode::ArrowRight),
        // Actions
        (PlayerActions::Jump, KeyCode::ShiftRight),
        (PlayerActions::SwapModel, KeyCode::Slash),
    ])
}
