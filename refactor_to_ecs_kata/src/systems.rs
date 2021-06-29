use bevy::prelude::*;

use crate::components::PlayerCharacter;
use crate::direction::Direction;
use crate::sprite::PLAYER_SPRITE;


pub fn load_sprite_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(PLAYER_SPRITE.path);
    let (texture_atlas, transform) = PlayerCharacter::get_texture_atlas(texture_handle);
    let texture_atlas = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas,
            transform,
            ..Default::default()
        })
        .insert(Timer::from_seconds(PLAYER_SPRITE.frame_time, true))
        .insert(PlayerCharacter::new());
}

pub fn input(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut PlayerCharacter>,
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

pub fn movement(
    time: Res<Time>,
    mut query: Query<(&mut PlayerCharacter, &mut Transform)>,
) {
    for (mut char, mut transform) in query.iter_mut() {
        let position = char.update_position(time.delta_seconds());
        let translation = &mut transform.translation;
        translation.x = position.x;
        translation.y = position.y;
    }
}

pub fn sprite(
    time: Res<Time>,
    mut query: Query<(&mut PlayerCharacter, &mut Timer, &mut TextureAtlasSprite)>,
) {
    for (mut char, mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            sprite.index = char.update_sprite_index();
        }
    }
}
