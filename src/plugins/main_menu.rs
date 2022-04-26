use std::{f32::consts::PI, time::Duration};

use crate::systems::{persistence, state_chooser};
use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetCollectionApp};
use bevy_tweening::{
    lens::{TransformRotateZLens, TransformScaleLens},
    *,
};
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
        .init_collection::<TitleScreenAssets>()
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::MainMenu)
                .with_system(state_chooser::state_chooser)
                .into(),
        );
    }
}

#[derive(AssetCollection)]
struct TitleScreenAssets {
    #[asset(path = "title_screen_bg.png")]
    bg: Handle<Image>,
    #[asset(path = "title_screen_just.png")]
    just: Handle<Image>,
    #[asset(path = "title_screen_break_out.png")]
    break_out: Handle<Image>,
}

fn create_menu(
    mut commands: Commands,
    state: Option<Res<CurrentState<GameState>>>,
    next_state: Option<Res<NextState<GameState>>>,
    title_screen_assets: Res<TitleScreenAssets>,
) {
    println!("Creating menu! {:?} {:?}", state, next_state);

    // spawn bg
    let bg_tween_rot = Tween::new(
        EaseFunction::QuadraticInOut,
        TweeningType::PingPong,
        Duration::from_secs(1),
        TransformRotateZLens {
            start: -PI / 4.0,
            end: PI / 4.0,
        },
    );
    let bg_tween_scale = Tween::new(
        EaseFunction::QuadraticInOut,
        TweeningType::PingPong,
        Duration::from_secs_f32(1.3),
        TransformScaleLens {
            start: Vec3::new(2.0, 2.0, 2.0),
            end: Vec3::new(2.3, 2.3, 2.3),
        },
    );
    commands
        .spawn_bundle(SpriteBundle {
            texture: title_screen_assets.bg.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Animator::new(Tracks::new([bg_tween_rot, bg_tween_scale])));

    // spawn "Break Out" text
    let break_out_tween_scale = Tween::new(
        EaseFunction::QuadraticInOut,
        TweeningType::PingPong,
        Duration::from_secs_f32(2.4),
        TransformScaleLens {
            start: Vec3::new(1.0, 1.0, 1.0),
            end: Vec3::new(1.1, 1.1, 1.1),
        },
    );
    commands
        .spawn_bundle(SpriteBundle {
            texture: title_screen_assets.break_out.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.1),
            ..default()
        })
        .insert(Animator::new(Tracks::new([break_out_tween_scale])));

    // spawn "Just" text
    let just_tween_rot = Tween::new(
        EaseFunction::QuadraticInOut,
        TweeningType::PingPong,
        Duration::from_secs(2),
        TransformRotateZLens {
            start: PI / 30.0,
            end: -PI / 30.0,
        },
    );
    let just_tween_scale = Tween::new(
        EaseFunction::QuadraticInOut,
        TweeningType::PingPong,
        Duration::from_secs_f32(1.3),
        TransformScaleLens {
            start: Vec3::new(1.0, 1.0, 1.0),
            end: Vec3::new(1.1, 1.1, 1.1),
        },
    );
    commands
        .spawn_bundle(SpriteBundle {
            texture: title_screen_assets.just.clone(),
            transform: Transform::from_xyz(-330.0, 120.0, 0.2),
            ..default()
        })
        .insert(Animator::new(Tracks::new([
            just_tween_rot,
            just_tween_scale,
        ])));
}

fn destroy_menu(
    mut _commands: Commands,
    state: Option<Res<CurrentState<GameState>>>,
    next_state: Option<Res<NextState<GameState>>>,
) {
    println!("Destroying menu! {:?} {:?}", state, next_state);
}

fn title_screen(mut commands: Commands) {}
