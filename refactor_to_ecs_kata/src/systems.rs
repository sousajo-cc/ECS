use bevy::prelude::*;

use crate::components::*;
use crate::direction::Direction;


pub fn load_player_sprite_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

    let texture_handle = asset_server.load(PLAYER_SPRITE.path);
    let (texture_atlas, transform) = PLAYER_SPRITE.get_texture_atlas(texture_handle);
    let texture_atlas = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas,
            transform,
            ..Default::default()
        })
        .insert(Timer::from_seconds(PLAYER_SPRITE.frame_time, true))
        .insert(PlayerCharacter)
        .insert(Position(Vec3::new(0.0,0.0,1.0)))
        .insert(Speed(0.0))
        .insert(Facing(Direction::Right))
        .insert(PLAYER_SPRITE);
}

pub fn input(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut PlayerCharacter,
                      &mut Speed,
                      &mut Facing,
                      &mut SpriteSheet)>,
) {
    for (mut _char, mut speed, mut facing, mut sprite) in query.iter_mut() {
        if input.pressed(KeyCode::Left) {
            set_direction(Direction::Left, &mut facing, &mut sprite);
            increase_speed(time.delta_seconds(), &mut speed);
        } else if input.pressed(KeyCode::Up) {
            set_direction(Direction::Up, &mut facing, &mut sprite);
            increase_speed(time.delta_seconds(), &mut speed);
        } else if input.pressed(KeyCode::Down) {
            set_direction(Direction::Down, &mut facing, &mut sprite);
            increase_speed(time.delta_seconds(), &mut speed);
        } else if input.pressed(KeyCode::Right) {
            set_direction(Direction::Right, &mut facing, &mut sprite);
            increase_speed(time.delta_seconds(), &mut speed);
        } else {
            decrease_speed(time.delta_seconds(), &mut speed);
        };
    }
}

pub fn movement(
    time: Res<Time>,
    mut query: Query<(
                      &mut Transform,
                      &mut Position,
                      &mut Speed,
                      &mut Facing)>,
) {
    for (mut transform, mut position, mut speed, mut facing) in query.iter_mut() {
        let position = update_position(time.delta_seconds(), &mut position, &mut speed, &mut facing);
        let translation = &mut transform.translation;
        translation.x = position.x;
        translation.y = position.y;
    }
}

pub fn sprite(
    time: Res<Time>,
    mut query: Query<(
                      &mut Timer,
                      &mut TextureAtlasSprite,
                      &mut Speed,
                      &mut Facing,
                      &mut SpriteSheet)>,
) {
    for (mut timer, mut sprite, mut speed, mut facing, mut spritesheet) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            sprite.index = update_sprite_index(&mut speed, &mut facing, &mut spritesheet);
        }
    }
}
