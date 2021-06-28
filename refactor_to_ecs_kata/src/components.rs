use bevy::prelude::*;

struct Sprite {
    path: &'static str,
    tile_size: (f32, f32),
    n_columns: usize,
    n_rows: usize,
    scale: f32,
    frame_time: f32,
}

const SPRITE: Sprite = Sprite {
    path: "character-walk.png",
    tile_size: (1.0, 2.0),
    n_columns: 4,
    n_rows: 4,
    scale: 36.0,
    frame_time: 0.2,
};

pub struct Char {
    position: Vec3,
    speed: f32,
    facing: Direction,
    sprite_index: u32,
}

impl Char {
    
    fn new() -> Self {
        Char {
            position: Vec3::new(0.0, 0.0, 1.0),
            speed: 0.0,
            facing: Direction::Right,
            sprite_index: 15,
        }
    }

    fn get_texture_atlas(texture_handle: Handle<Texture>) -> (TextureAtlas, Transform) {
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::from(SPRITE.tile_size),
            SPRITE.n_columns,
            SPRITE.n_rows,
        );
        let transform = Transform::from_scale(Vec3::splat(SPRITE.scale));
        (texture_atlas, transform)
    }

    fn update_sprite_index(&mut self) -> u32 {
        if self.speed == 0.0 {
            self.sprite_index = self.facing.still_sprite_index();
        } else {
            self.sprite_index += 1;
            if self.sprite_index % SPRITE.n_columns as u32 == 0 {
                self.sprite_index -= SPRITE.n_columns as u32;
            }
        };
        self.sprite_index
    }

    fn set_direction(&mut self, direction: Direction) {
        if self.facing != direction {
            self.facing = direction;
            self.sprite_index = self.facing.still_sprite_index();
        };
    }

    fn decrease_speed(&mut self, dt: f32) {
        self.speed -= 240.0 * dt;
        if self.speed < 0.0 {
            self.speed = 0.0;
        };
    }

    fn increase_speed(&mut self, dt: f32) {
        self.speed += 80.0 * dt;
        if self.speed > 120.0 {
            self.speed = 120.0;
        };
    }

    fn update_position(&mut self, dt: f32) -> Vec3 {
        let ds = dt * self.speed * self.facing.as_vec();
        self.position += ds;
        self.position
    }

}