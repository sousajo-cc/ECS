use bevy::prelude::*;

use crate::direction::Direction;


pub struct PlayerCharacter;

pub struct Position(pub Vec3);
pub struct Speed(pub f32);
pub struct Facing(pub Direction);
pub struct SpriteSheet {
    pub path: &'static str,
    pub tile_size: (f32, f32),
    pub n_columns: usize,
    pub n_rows: usize,
    pub scale: f32,
    pub frame_time: f32,
    pub current_index: u32,
}

pub const PLAYER_SPRITE: SpriteSheet = SpriteSheet {
    path: "character-walk.png",
    tile_size: (1.0, 2.0),
    n_columns: 4,
    n_rows: 4,
    scale: 36.0,
    frame_time: 0.2,
    current_index: 15,
};

impl SpriteSheet {
    pub fn get_texture_atlas(&self, texture_handle: Handle<Texture>) -> (TextureAtlas, Transform) {
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::from(self.tile_size),
            self.n_columns,
            self.n_rows,
        );
        let transform = Transform::from_scale(Vec3::splat(self.scale));
        (texture_atlas, transform)
    }
}


pub fn update_sprite_index( speed: &mut Speed, facing: &mut Facing, sprite: &mut SpriteSheet) -> u32 {
    if speed.0 == 0.0 {
        sprite.current_index = still_sprite_index(facing);
    } else {
        sprite.current_index += 1;
        if sprite.current_index % sprite.n_columns as u32 == 0 {
            sprite.current_index -= sprite.n_columns as u32;
        }
    };
    sprite.current_index
}

pub fn set_direction( direction: Direction, facing: &mut Facing, sprite: &mut SpriteSheet) {
    if facing.0 != direction {
        facing.0 = direction;
        sprite.current_index = still_sprite_index(facing);
    };
}

pub fn decrease_speed( dt: f32, speed: &mut Speed) {
    speed.0 -= 240.0 * dt;
    if speed.0 < 0.0 {
        speed.0 = 0.0;
    };
}

pub fn increase_speed( dt: f32, speed: &mut Speed) {
    speed.0 += 80.0 * dt;
    if speed.0 > 120.0 {
        speed.0 = 120.0;
    };
}

pub fn update_position( dt: f32, position: &mut Position, speed: &mut Speed, facing: &mut Facing) -> Vec3 {
    let ds = dt * speed.0 * facing.0.as_vec();
    position.0 += ds;
    position.0
}

fn still_sprite_index(facing: &mut Facing) -> u32 {
    match facing.0 {
        Direction::Down => 0,
        Direction::Up => 4,
        Direction::Left => 8,
        Direction::Right => 13, //wtf
    }
}

