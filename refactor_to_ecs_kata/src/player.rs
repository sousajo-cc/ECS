use bevy::prelude::*;

use crate::direction::Direction;

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

fn load_sprite_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(SPRITE.path);
    let (texture_atlas, transform) = Char::get_texture_atlas(texture_handle);
    let texture_atlas = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas,
            transform,
            ..Default::default()
        })
        .insert(Timer::from_seconds(SPRITE.frame_time, true))
        .insert(Char::new());
}

fn input(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Char>, //if you forget mut, the compiler panics lol
) {
    for mut char in query.iter_mut() {
        if input.pressed(KeyCode::Left) {
            char.set_direction(Direction::Left);
            char.increase_speed(time.delta_seconds());
        } else if input.pressed(KeyCode::Up) {
            char.set_direction(Direction::Up);
            char.increase_speed(time.delta_seconds());
        } else if input.pressed(KeyCode::Down) {
            char.set_direction(Direction::Down);
            char.increase_speed(time.delta_seconds());
        } else if input.pressed(KeyCode::Right) {
            char.set_direction(Direction::Right);
            char.increase_speed(time.delta_seconds());
        } else {
            char.decrease_speed(time.delta_seconds());
        };
    }
}

fn movement(
    time: Res<Time>,
    mut query: Query<(&mut Char, &mut Transform)>,
) {
    for (mut char, mut transform) in query.iter_mut() {
        let position = char.update_position(time.delta_seconds());
        let translation = &mut transform.translation;
        translation.x = position.x;
        translation.y = position.y;
    }
}

fn sprite(
    time: Res<Time>,
    mut query: Query<(&mut Char, &mut Timer, &mut TextureAtlasSprite)>,
) {
    for (mut char, mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            sprite.index = char.update_sprite_index();
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(load_sprite_sheet.system())
            .add_system(input.system())
            .add_system(movement.system())
            .add_system(sprite.system());
    }
}
