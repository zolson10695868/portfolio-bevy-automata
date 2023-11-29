mod cell;
mod grid;
mod rendering;
mod rule;

use bevy::prelude::*;
use rendering::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CustomMaterialPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_cubes, color_cubes))
        .run();
}

// temporary systems; proof of concept to show that data can be mutated and be represented in the
// shader

fn move_cubes(mut instance: Query<&mut InstanceMaterialData>, time: Res<Time>) {
    for mut i in instance.iter_mut() {
        i.0.iter_mut().enumerate().for_each(|(i, c)| {
            let vel = (time.elapsed_seconds() + i as f32).cos() / 8.;
            c.position.x += vel * time.delta_seconds();
        });
    }
}

fn color_cubes(mut instance: Query<&mut InstanceMaterialData>, time: Res<Time>) {
    for mut i in instance.iter_mut() {
        i.0.iter_mut().for_each(|c| {
            c.color[0] = (c.color[0] + 0.4 * time.delta_seconds()).rem_euclid(1.);
            c.color[1] = (c.color[1] - 0.05 * time.delta_seconds()).rem_euclid(1.);
            c.color[2] = (c.color[2] + 1.13 * time.delta_seconds()).rem_euclid(1.);
        })
    }
}
