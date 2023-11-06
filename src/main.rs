mod rendering;
use bevy::prelude::{App, DefaultPlugins, Startup};
use rendering::{setup, CustomMaterialPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CustomMaterialPlugin))
        .add_systems(Startup, setup)
        .run();
}
