use bevy::prelude::*;

use crate::direction::Direction;
use crate::sprite_sheet::SpriteSheet;


pub struct PlayerCharacter {
    position: Vec3,
    speed: f32,
    facing: Direction,
    sprite: SpriteSheet,
}

impl PlayerCharacter {

    pub fn new() -> Self {
        use crate::sprite_sheet::PLAYER_SPRITE;

        PlayerCharacter {
            position: Vec3::new(0.0, 0.0, 1.0),
            speed: 0.0,
            facing: Direction::Right,
            sprite: PLAYER_SPRITE,
        }
    }

    pub fn update_sprite_index(&mut self) -> u32 {
        if self.speed == 0.0 {
            self.sprite.current_index = self.still_sprite_index();
        } else {
            self.sprite.current_index += 1;
            if self.sprite.current_index % self.sprite.n_columns as u32 == 0 {
                self.sprite.current_index -= self.sprite.n_columns as u32;
            }
        };
        self.sprite.current_index
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if self.facing != direction {
            self.facing = direction;
            self.sprite.current_index = self.still_sprite_index();
        };
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

    pub fn update_position(&mut self, dt: f32) -> Vec3 {
        let ds = dt * self.speed * self.facing.as_vec();
        self.position += ds;
        self.position
    }

    fn still_sprite_index(&self) -> u32 {
        match &self.facing {
            Direction::Down => 0,
            Direction::Up => 4,
            Direction::Left => 8,
            Direction::Right => 13, //wtf
        }
    }

}