// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use crate::states::AppState;
use bevy::prelude::{
    App, OnEnter, Plugin,
};

pub mod states;
mod systems;
use self::{
     systems::game_setup, states::GameState
};


use super::{
    debug::DebugPlugin,
    camera::CameraPlugin, 
    tilemap::TilemapPlugin,
    player::PlayerPlugin,
    animation::AnimationPlugin,
    //enemy::EnemyPlugin,
};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
        .add_plugins(CameraPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(TilemapPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(AnimationPlugin)
        .add_systems(OnEnter(AppState::InGame), game_setup);
    }
}
