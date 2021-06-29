use bevy::prelude::*;

pub struct Sprite {
    pub path: &'static str,
    pub tile_size: (f32, f32),
    pub n_columns: usize,
    pub n_rows: usize,
    pub scale: f32,
    pub frame_time: f32,
    pub current_index: u32,
}

pub const PLAYER_SPRITE: Sprite = Sprite {
    path: "character-walk.png",
    tile_size: (1.0, 2.0),
    n_columns: 4,
    n_rows: 4,
    scale: 36.0,
    frame_time: 0.2,
    current_index: 15,
};

impl Sprite {
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