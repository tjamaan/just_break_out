use crate::GameState;
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
                .with_system(gameplay)
                .into(),
        );
    }
}

fn gameplay(mut commands: Commands, mut egui_context: ResMut<EguiContext>) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        egui::Area::new("buttons")
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ui.ctx(), |ui| {
                ui.vertical_centered(|ui| {
                    if ui.button("Gamepleigh!").clicked() {
                        commands.insert_resource(NextState(GameState::GameOver));
                    }
                    if ui.button("Quit").clicked() {
                        println!("Eh... maybe next time.");
                    }
                });
            });
    });
}
