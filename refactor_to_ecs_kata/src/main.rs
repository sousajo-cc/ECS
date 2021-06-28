use bevy::prelude::*;

mod direction;
mod player;

fn main() {
    use player::*;

    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .run();
}
