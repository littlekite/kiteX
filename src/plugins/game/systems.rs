// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{
        Commands, DespawnRecursiveExt, Entity, EventReader, Name, Input, KeyCode, NextState, Query, Res,
        ResMut, Vec2,Vec3, Without, AssetServer, Assets, Resource, Handle,
    },
    window::{Window, WindowFocused}, sprite::{TextureAtlas, TextureAtlasSprite}, utils::HashMap,
};

use crate::plugins::{
    camera::bundles::GameCameraBundle,
    atlas::resources::GameAtlases,
    tilemap::bundles::TilemapBundle,
    player::{bundles::PlayerBundle, self}
};

use super::states::{GameState};
use bevy_xpbd_2d::prelude::*;


pub fn game_setup(
    mut commands: Commands,
    game_atlases: Res<GameAtlases>,
    mut game_state: ResMut<NextState<GameState>>
) {

    commands.spawn(GameCameraBundle::new(0.8));
  
    commands
    .spawn((PlayerBundle::new(
        game_atlases.player.clone(),
        Vec2 { x: 32.0, 
            y: 32.0 },
        //Vec2 { x: 230.0, y: 150.0 },
        Vec2 { x: 230.0, y: 166.0 }, // 150+16(脚)
        15.0,
        65.0,
        18.0,
        10.0,
        90.0,
        150.0,
        0.2,
    )));


    commands
        .spawn(TilemapBundle::new(
            game_atlases.tileset.clone(),
            (32.0, 32.0).into(),
            (20, 10).into(),
            vec![
                //vec![91,92,93,94,95,96,97,98,99,100,101,102,103,104,105],
                vec![181,182,183,184,185,186,187,188,189,190,191,192,193,194,195,196,197,198,199,199],
                vec![161,162,163,164,165,166,167,168,169,170,171,172,173,174,175,176,177,178,179,180],
                vec![141,142,143,144,145,146,147,148,149,150,151,152,153,154,155,156,157,158,159,160],
                vec![121,122,123,124,125,126,127,128,129,130,131,132,133,134,135,136,137,138,139,140],
                vec![101,102,103,104,105,106,107,108,109,110,111,112,113,114,115,116,117,118,119,120],
                vec![81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99,100],
                vec![61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80],
                vec![41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60],
                vec![21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40],
                vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]
            ],
            vec![],
            None,
        ))
        .insert(Name::from("OverworldTilemap"));
    game_state.set(GameState::Running);
}
