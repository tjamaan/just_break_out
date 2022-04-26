use crate::{systems::state_chooser, GameState};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use iyes_loopless::prelude::*;

pub(crate) struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        let enter_gameover = SystemStage::parallel();
        let exit_gameover = SystemStage::parallel();

        app.stage(
            "TransitionStage",
            |stage: &mut StateTransitionStage<crate::GameState>| {
                stage.set_enter_stage(GameState::GameOver, enter_gameover);
                stage.set_exit_stage(GameState::GameOver, exit_gameover);
                stage
            },
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::GameOver)
                .with_system(state_chooser::state_chooser)
                .into(),
        );
    }
}
