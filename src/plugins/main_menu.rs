use crate::systems::persistence;
use crate::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use iyes_loopless::prelude::*;

pub(crate) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        let enter_menu = SystemStage::parallel().with_system(create_menu);
        let exit_menu = SystemStage::parallel()
            .with_system(destroy_menu)
            .with_system(persistence::destroy_nonpersistent_entities);

        app.stage(
            "TransitionStage",
            |stage: &mut StateTransitionStage<crate::GameState>| {
                stage.set_enter_stage(GameState::MainMenu, enter_menu);
                stage.set_exit_stage(GameState::MainMenu, exit_menu);
                stage
            },
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::MainMenu)
                .with_system(main_menu)
                .into(),
        );
    }
}

fn create_menu(
    mut _commands: Commands,
    state: Res<CurrentState<GameState>>,
    next_state: Res<NextState<GameState>>,
) {
    println!("Creating menu! {:?} {:?}", state, next_state);
}

fn destroy_menu(
    mut _commands: Commands,
    state: Res<CurrentState<GameState>>,
    next_state: Res<NextState<GameState>>,
) {
    println!("Destroying menu! {:?} {:?}", state, next_state);
}

fn main_menu(mut commands: Commands, mut egui_context: ResMut<EguiContext>) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        egui::Area::new("buttons")
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ui.ctx(), |ui| {
                ui.vertical_centered(|ui| {
                    if ui.button("Play").clicked() {
                        commands.insert_resource(NextState(GameState::Gameplay));
                    }
                    if ui.button("Quit").clicked() {
                        println!("Eh... maybe next time.");
                    }
                });
            });
    });
}
