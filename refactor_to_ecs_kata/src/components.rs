use bevy::prelude::*;

use crate::direction::Direction;
use crate::sprite_sheet::SpriteSheet;


pub struct PlayerCharacter {
    pub sprite: SpriteSheet,
}

pub struct Position(pub Vec3);
pub struct Speed(pub f32);
pub struct Facing(pub Direction);

impl PlayerCharacter {

    pub fn new() -> Self {
        use crate::sprite_sheet::PLAYER_SPRITE;

        PlayerCharacter {
            sprite: PLAYER_SPRITE,
        }
    }

    pub fn update_sprite_index(&mut self, speed: &mut Speed, facing: &mut Facing) -> u32 {
        if speed.0 == 0.0 {
            self.sprite.current_index = self.still_sprite_index(facing);
        } else {
            self.sprite.current_index += 1;
            if self.sprite.current_index % self.sprite.n_columns as u32 == 0 {
                self.sprite.current_index -= self.sprite.n_columns as u32;
            }
        };
        self.sprite.current_index
    }

    pub fn set_direction(&mut self, direction: Direction, facing: &mut Facing) {
        if facing.0 != direction {
            facing.0 = direction;
            self.sprite.current_index = self.still_sprite_index(facing);
        };
    }

    pub fn decrease_speed(&mut self, dt: f32, speed: &mut Speed) {
        speed.0 -= 240.0 * dt;
        if speed.0 < 0.0 {
            speed.0 = 0.0;
        };
    }

    pub fn increase_speed(&mut self, dt: f32, speed: &mut Speed) {
        speed.0 += 80.0 * dt;
        if speed.0 > 120.0 {
            speed.0 = 120.0;
        };
    }

    pub fn update_position(&mut self, dt: f32, position: &mut Position, speed: &mut Speed, facing: &mut Facing) -> Vec3 {
        let ds = dt * speed.0 * facing.0.as_vec();
        position.0 += ds;
        position.0
    }

    fn still_sprite_index(&self, facing: &mut Facing) -> u32 {
        match facing.0 {
            Direction::Down => 0,
            Direction::Up => 4,
            Direction::Left => 8,
            Direction::Right => 13, //wtf
        }
    }

}