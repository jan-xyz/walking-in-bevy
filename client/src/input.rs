use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use walking_in_bevy_shared::player::PlayerActions;

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
