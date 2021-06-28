use bevy::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Direction {
    Down,
    Up,
    Left,
    Right,
}

impl Direction {
    //TODO: move to player
    pub fn still_sprite_index(&self) -> u32 {
        match &self {
            Direction::Down => 0,
            Direction::Up => 4,
            Direction::Left => 8,
            Direction::Right => 13, //wtf
        }
    }
    pub fn as_vec(&self) -> Vec3 {
        match &self {
            Direction::Down => Vec3::new(0.0, -1.0, 0.0),
            Direction::Up => Vec3::new(0.0, 1.0, 0.0),
            Direction::Left => Vec3::new(-1.0, 0.0, 0.0),
            Direction::Right => Vec3::new(1.0, 0.0, 0.0),
        }
    }
}