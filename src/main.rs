mod plugins;
mod systems;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_tweening::TweeningPlugin;
use iyes_loopless::prelude::*;
use plugins::{GameOverPlugin, GameplayPlugin, MainMenuPlugin};
use systems::persistence::Persistent;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    MainMenu,
    Gameplay,
    GameOver,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Just Break Out!".into(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::TEAL))
        .add_plugins(DefaultPlugins)
        .add_plugin(TweeningPlugin)
        .add_plugin(EguiPlugin)
        .add_stage_after(
            CoreStage::PreUpdate,
            "TransitionStage",
            StateTransitionStage::new(GameState::MainMenu),
        )
        .add_startup_system(setup_camera)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GameplayPlugin)
        .add_plugin(GameOverPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Persistent);
}
