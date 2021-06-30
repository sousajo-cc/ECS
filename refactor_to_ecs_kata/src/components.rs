use bevy::prelude::*;

use crate::direction::Direction;
use crate::sprite::Sprite;


pub struct PlayerCharacter {
    pub sprite: Sprite,
}

pub struct Facing(pub Direction);
pub struct Position(pub Vec3);
pub struct Speed(pub f32);

impl PlayerCharacter {

    pub fn new() -> Self {
        use crate::sprite::PLAYER_SPRITE;

        PlayerCharacter {
            sprite: PLAYER_SPRITE,
        }
    }








}