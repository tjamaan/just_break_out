use crate::GameState;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy_egui::{egui, EguiContext};

pub(crate) struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        let mut enter_gameover = SystemStage::parallel();
        let mut exit_gameover = SystemStage::parallel();

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
                .with_system(gameover)
                .into(),
        );
    }
}

fn gameover(mut commands: Commands, mut egui_context: ResMut<EguiContext>) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        egui::Area::new("buttons")
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ui.ctx(), |ui| {
                ui.vertical_centered(|ui| {
                    if ui.button("Game Over /YEAAAAH/").clicked() {
                        commands.insert_resource(NextState(GameState::MainMenu));
                    }
                    if ui.button("Quit").clicked() {
                        println!("Eh... maybe next time.");
                    }
                });
            });
    });
}
