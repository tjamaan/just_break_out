use crate::{systems::state_chooser, GameState};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use iyes_loopless::prelude::*;

pub(crate) struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        let enter_gameplay = SystemStage::parallel();
        let exit_gameplay = SystemStage::parallel();

        app.stage(
            "TransitionStage",
            |stage: &mut StateTransitionStage<crate::GameState>| {
                stage.set_enter_stage(GameState::Gameplay, enter_gameplay);
                stage.set_exit_stage(GameState::Gameplay, exit_gameplay);
                stage
            },
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Gameplay)
                .with_system(state_chooser::state_chooser)
                .into(),
        );
    }
}
