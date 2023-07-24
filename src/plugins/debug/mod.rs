// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::Plugin;
use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};

use bevy_inspector_egui::{prelude::*, bevy_inspector, DefaultInspectorConfigPlugin};

use crate::states::AppState;
use bevy_egui::*;

use crate::plugins::{
    game::states::GameState
};

use bevy::prelude::*;
use egui::containers::Frame;
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        #[cfg(debug_assertions)]
        app.add_plugins(WorldInspectorPlugin::new())
            .add_plugins(StateInspectorPlugin::<AppState>::default())
            .add_plugins(StateInspectorPlugin::<GameState>::default());
    }
}

