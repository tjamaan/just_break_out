mod plugins;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy_egui::EguiPlugin;
use plugins::{MainMenuPlugin, GameplayPlugin, GameOverPlugin};

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
        .add_plugin(EguiPlugin)
        .add_stage_after(
            CoreStage::PreUpdate,
            "TransitionStage",
            StateTransitionStage::new(GameState::MainMenu)
        )
        .add_plugin(MainMenuPlugin)
        .add_plugin(GameplayPlugin)
        .add_plugin(GameOverPlugin)
        .run();
}