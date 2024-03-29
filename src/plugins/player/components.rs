// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Component, Vec2},
    reflect::Reflect,
    time::Timer,
};


#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct Player;

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Reflect)]
pub struct PlayerSize(pub Vec2);

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Reflect)]
pub struct PlayerWalkSpeed(pub f32);

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Reflect)]
pub struct PlayerVelocity(pub Vec2);

#[derive(Component, Default, Debug, Clone, Reflect)]
pub struct PlayerHealth(pub f32);

#[derive(Component, Default, Debug, Clone, Reflect)]
pub struct PlayerHealthMax(pub f32);

#[derive(Component, Default, Debug, Clone, Reflect)]
pub struct PlayerMeleeAttackDamage(pub f32);

#[derive(Component, Default, Debug, Clone, Reflect)]
pub struct PlayerMeleeAttackTimer(pub Timer);

#[derive(Component, Default, Debug, Clone, Reflect)]
pub struct PlayerMeleeAttackHitPosition(pub Vec2);

#[derive(Component, Default, Debug, Clone, Reflect)]
pub struct PlayerMagicAttackTimer(pub Timer);

#[derive(Component, Default, Debug, Clone, Reflect)]
pub struct PlayerMagicAttackDamage(pub f32);

#[derive(Component, Default, Debug, Clone, Reflect)]
pub struct PlayerMagicAttackSpeed(pub f32);

#[derive(Component, Default, Debug, Clone, Reflect)]
pub struct PlayerIsStunnedTimer(pub Timer);

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum PlayerDirection {
    #[default]
    Right,
    Down,
    Left,
    Up,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerDebuffSlowWalk;

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerDebuffNoMagic;

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerDebuffNoDamageAgainstBlobs;

#[derive(Component, Default, Debug, Clone, PartialEq)]
pub struct TilemapRoad(pub Vec<Vec<f32>>);
