use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::shared::player::PlayerActions;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerActions>::default());
    }
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
