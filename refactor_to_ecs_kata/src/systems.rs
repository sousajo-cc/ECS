use bevy::prelude::*;

use crate::components::{PlayerCharacter, Facing, Position};
use crate::direction::Direction;


pub fn load_player_sprite_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    use crate::sprite::PLAYER_SPRITE;

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
        .insert(Facing(Direction::Right))
        .insert(Position(Vec3::new(0.0, 0.0, 1.0)));
}
pub fn still_sprite_index(facing: &mut Facing) -> u32 {
    match &facing.0 {
        Direction::Down => 0,
        Direction::Up => 4,
        Direction::Left => 8,
        Direction::Right => 13, //wtf
    }
}

pub fn set_direction(player: &mut PlayerCharacter, facing: &mut Facing, direction: Direction) {
    if facing.0 != direction {
        facing.0 = direction;
        player.sprite.current_index = still_sprite_index(facing);
    };
}

pub fn update_sprite_index(player: &mut PlayerCharacter, facing: &mut Facing) -> u32 {
    if player.speed == 0.0 {
        player.sprite.current_index = still_sprite_index(facing);
    } else {
        player.sprite.current_index += 1;
        if player.sprite.current_index % player.sprite.n_columns as u32 == 0 {
            player.sprite.current_index -= player.sprite.n_columns as u32;
        }
    };
    player.sprite.current_index
}

pub fn input(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut PlayerCharacter, &mut Facing)>,
) {
    for (mut char, mut facing) in query.iter_mut() {
        if input.pressed(KeyCode::Left) {
            set_direction(&mut char, &mut facing, Direction::Left);
            char.increase_speed(time.delta_seconds());
        } else if input.pressed(KeyCode::Up) {
            set_direction(&mut char, &mut facing, Direction::Up);
            char.increase_speed(time.delta_seconds());
        } else if input.pressed(KeyCode::Down) {
            set_direction(&mut char, &mut facing, Direction::Down);
            char.increase_speed(time.delta_seconds());
        } else if input.pressed(KeyCode::Right) {
            set_direction(&mut char, &mut facing, Direction::Right);
            char.increase_speed(time.delta_seconds());
        } else {
            char.decrease_speed(time.delta_seconds());
        };
    }
}

pub fn update_position(player: &mut PlayerCharacter, facing: &mut Facing, position: &mut Position, dt: f32) -> Vec3 {
    let ds = dt * player.speed * facing.0.as_vec();
    position.0 += ds;
    position.0
}

    pub fn movement(
    time: Res<Time>,
    mut query: Query<(&mut PlayerCharacter, &mut Transform, &mut Facing, &mut Position)>,
) {
    for (mut char, mut transform, mut facing, mut position) in query.iter_mut() {
        let position = update_position(&mut char, &mut facing, &mut position, time.delta_seconds());
        let translation = &mut transform.translation;
        translation.x = position.x;
        translation.y = position.y;
    }
}


pub fn sprite(
    time: Res<Time>,
    mut query: Query<(&mut PlayerCharacter, &mut Timer, &mut TextureAtlasSprite, &mut Facing)>,
) {
    for (mut char, mut timer, mut sprite, mut facing) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            sprite.index = update_sprite_index(&mut char, &mut facing);
        }
    }
}
