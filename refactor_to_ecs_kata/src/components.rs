use bevy::prelude::*;

use crate::direction::Direction;
use crate::sprite::Sprite;


pub struct PlayerCharacter {
    pub speed: f32,
    pub sprite: Sprite,
}

pub struct Facing(pub Direction);
pub struct Position(pub Vec3);
pub struct Speed(pub f32);

impl PlayerCharacter {

    pub fn new() -> Self {
        use crate::sprite::PLAYER_SPRITE;

        PlayerCharacter {
            speed: 0.0,
            sprite: PLAYER_SPRITE,
        }
    }




    pub fn decrease_speed(&mut self, dt: f32) {
        self.speed -= 240.0 * dt;
        if self.speed < 0.0 {
            self.speed = 0.0;
        };
    }

    pub fn increase_speed(&mut self, dt: f32) {
        self.speed += 80.0 * dt;
        if self.speed > 120.0 {
            self.speed = 120.0;
        };
    }



}