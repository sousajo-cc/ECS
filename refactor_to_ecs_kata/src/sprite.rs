pub struct Sprite {
    pub path: &'static str,
    pub tile_size: (f32, f32),
    pub n_columns: usize,
    pub n_rows: usize,
    pub scale: f32,
    pub frame_time: f32,
}

pub const SPRITE: Sprite = Sprite {
    path: "character-walk.png",
    tile_size: (1.0, 2.0),
    n_columns: 4,
    n_rows: 4,
    scale: 36.0,
    frame_time: 0.2,
};
