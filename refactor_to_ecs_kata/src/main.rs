use bevy::prelude::*;

mod direction;
mod components;
mod systems;

fn main() {
    use systems::*;

    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(load_sprite_sheet.system())
        .add_system(input.system())
        .add_system(movement.system())
        .add_system(sprite.system())
        .run();
}
