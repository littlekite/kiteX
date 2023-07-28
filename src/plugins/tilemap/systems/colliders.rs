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
    for (tile_transform, tile_sprite) in added_tiles_query.iter() {
        let (mut tilemap_colliders, tilemap_solid_indices, tilemap_tile_size) = tilemap_query
            .get_single_mut()
            .expect("0 or more than 1 tilemap111 found.");

        let tile_texture_index = tile_sprite.index as u32;
        if tilemap_solid_indices
            .0
            .iter()
            .find(|&&e| e == tile_texture_index)
            .is_some()
        {
            let tile_center_in_world = tile_transform.translation.truncate();

            tilemap_colliders.0.push(Rect::from_center_size(
                tile_center_in_world,
                tilemap_tile_size.0,
            ));
            
        }
    }
    /*
    // Draw the colliders in debug-mode.
    #[cfg(debug_assertions)]
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::RED.with_a(100.0),
                custom_size: Some(Vec2::new(2.0,2.0)),
                ..default()
            },
            transform:Transform::from_xyz(200.0,155.0,100.),
            ..default()
        })
        .insert(Name::from("Collider1"));
        // Draw the colliders in debug-mode.
    #[cfg(debug_assertions)]
    commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED.with_a(100.0),
                    custom_size: Some(Vec2::new(2.0,2.0)),
                    ..default()
                },
                transform:Transform::from_xyz(275.0,155.0,100.),
                ..default()
            })
            .insert(Name::from("Collider2"));

    #[cfg(debug_assertions)]
    commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED.with_a(100.0),
                    custom_size: Some(Vec2::new(2.0,2.0)),
                    ..default()
                },
                transform:Transform::from_xyz(290.0,145.0,100.),
                ..default()
            })
            .insert(Name::from("Collider3"));

    #[cfg(debug_assertions)]
    commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED.with_a(100.0),
                    custom_size: Some(Vec2::new(2.0,2.0)),
                    ..default()
                },
                transform:Transform::from_xyz(310.0,125.0,100.),
                ..default()
            })
            .insert(Name::from("Collider2"));
    #[cfg(debug_assertions)]
    commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED.with_a(100.0),
                    custom_size: Some(Vec2::new(2.0,2.0)),
                    ..default()
                },
                transform:Transform::from_xyz(330.0,110.0,100.),
                ..default()
            })
            .insert(Name::from("Collider3"));
    #[cfg(debug_assertions)]
        commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED.with_a(100.0),
                        custom_size: Some(Vec2::new(2.0,2.0)),
                        ..default()
                    },
                    transform:Transform::from_xyz(450.0,110.0,100.),
                    ..default()
                })
                .insert(Name::from("Collider3"));
    #[cfg(debug_assertions)]
            commands
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::RED.with_a(100.0),
                            custom_size: Some(Vec2::new(2.0,2.0)),
                            ..default()
                        },
                        transform:Transform::from_xyz(500.0,135.0,100.),
                        ..default()
                    })
                    .insert(Name::from("Collider3"));
    */
    
}
