use crate::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use iyes_loopless::prelude::*;

pub(crate) fn state_chooser(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    current_state: Res<CurrentState<GameState>>,
) {
    egui::Window::new("Switch State").show(egui_context.ctx_mut(), |ui| {
        ui.vertical(|ui| {
            ui.label(format!("Current state: {:?}", current_state.0));
            if ui.button("MainMenu").clicked() {
                commands.insert_resource(NextState(GameState::MainMenu));
            }
            if ui.button("Gameplay").clicked() {
                commands.insert_resource(NextState(GameState::Gameplay));
            }
            if ui.button("GameOver").clicked() {
                commands.insert_resource(NextState(GameState::GameOver));
            }
        });
    });
}
