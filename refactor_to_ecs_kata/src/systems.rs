use bevy::prelude::*;

use crate::components::*;
use crate::direction::Direction;


pub fn load_player_sprite_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    use crate::sprite_sheet::PLAYER_SPRITE;

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
        .insert(PlayerCharacter::new())
        .insert(Position(Vec3::new(0.0,0.0,1.0)))
        .insert(Speed(0.0))
        .insert(Facing(Direction::Right));
}

pub fn input(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut PlayerCharacter, &mut Speed, &mut Facing)>,
) {
    for (mut char, mut speed, mut facing) in query.iter_mut() {
        if input.pressed(KeyCode::Left) {
            char.set_direction(Direction::Left, &mut facing);
            char.increase_speed(time.delta_seconds(), &mut speed);
        } else if input.pressed(KeyCode::Up) {
            char.set_direction(Direction::Up, &mut facing);
            char.increase_speed(time.delta_seconds(), &mut speed);
        } else if input.pressed(KeyCode::Down) {
            char.set_direction(Direction::Down, &mut facing);
            char.increase_speed(time.delta_seconds(), &mut speed);
        } else if input.pressed(KeyCode::Right) {
            char.set_direction(Direction::Right, &mut facing);
            char.increase_speed(time.delta_seconds(), &mut speed);
        } else {
            char.decrease_speed(time.delta_seconds(), &mut speed);
        };
    }
}

pub fn movement(
    time: Res<Time>,
    mut query: Query<(&mut PlayerCharacter,
                      &mut Transform,
                      &mut Position,
                      &mut Speed,
                      &mut Facing)>,
) {
    for (mut char, mut transform, mut position, mut speed, mut facing) in query.iter_mut() {
        let position = char.update_position(time.delta_seconds(), &mut position, &mut speed, &mut facing);
        let translation = &mut transform.translation;
        translation.x = position.x;
        translation.y = position.y;
    }
}

pub fn sprite(
    time: Res<Time>,
    mut query: Query<(&mut PlayerCharacter, &mut Timer, &mut TextureAtlasSprite, &mut Speed, &mut Facing)>,
) {
    for (mut char, mut timer, mut sprite, mut speed, mut facing) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            sprite.index = char.update_sprite_index(&mut speed, &mut facing);
        }
    }
}
