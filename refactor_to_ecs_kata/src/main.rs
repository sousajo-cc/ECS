use bevy::prelude::*;

mod components;
mod direction;
mod sprite;
mod systems;

fn main() {
    use systems::*;

    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(load_player_sprite_sheet.system())
        .add_system(input.system())
        .add_system(movement.system())
        .add_system(sprite.system())
        .run();
}
