// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::*,
    sprite::{TextureAtlasSprite, SpriteBundle, Sprite},
};

use crate::plugins::tilemap::components::{
    Tilemap, TilemapColliders, TilemapSolidTextureIndices, TilemapTile, TilemapTileSize,
};

pub fn spawn_colliders(
    mut commands: Commands,
    mut tilemap_query: Query<
        (
            &mut TilemapColliders,
            &TilemapSolidTextureIndices,
            &TilemapTileSize,
        ),
        With<Tilemap>,
    >,
    added_tiles_query: Query<(&Transform, &TextureAtlasSprite), Added<TilemapTile>>,
) {

    
}
