use bevy::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Direction {
    Down,
    Up,
    Left,
    Right,
}

impl Direction {
    pub fn as_vec(&self) -> Vec3 {
        match &self {
            Direction::Down => Vec3::new(0.0, -1.0, 0.0),
            Direction::Up => Vec3::new(0.0, 1.0, 0.0),
            Direction::Left => Vec3::new(-1.0, 0.0, 0.0),
            Direction::Right => Vec3::new(1.0, 0.0, 0.0),
        }
    }
}