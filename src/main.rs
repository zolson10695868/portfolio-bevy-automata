mod cell;
mod grid;
mod rendering;
mod rule;

use std::time::Duration;

use bevy::{
    prelude::*,
    render::view::NoFrustumCulling,
    tasks::{block_on, AsyncComputeTaskPool, Task},
    window::close_on_esc,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use grid::{Grid, MainGrid};
use rendering::*;
use rule::{Neighbors, Rule};

#[derive(Resource)]
struct GridTimer(Timer);

fn main() {
    App::new()
        .insert_resource(Rule {
            survival: vec![4..5],
            birth: vec![4..5],
            states: 5,
            neighbors: Neighbors::Moore,
        })
        .insert_resource(GridTimer(Timer::new(
            Duration::from_millis(200),
            TimerMode::Repeating,
        )))
        .add_event::<GridReset>()
        .add_plugins((DefaultPlugins, CustomMaterialPlugin, EguiPlugin))
        .add_systems(Startup, create_grid)
        //.add_systems(Startup, setup)
        //.add_systems(Update, (move_cubes, color_cubes))
        .add_systems(Update, (update_grid, render_grid_data, rotate_g))
        .add_systems(Update, close_on_esc)
        .add_systems(Update, draw_window)
        .run();
}

#[derive(Event)]
struct GridReset;

fn draw_window(
    mut contexts: EguiContexts,
    mut rule: ResMut<Rule>,
    mut rule_str: Local<String>,
    mut ev: EventWriter<GridReset>,
) {
    if rule_str.is_empty() {
        *rule_str = "4/4/5/M".into();
    }
    egui::Window::new("Settings")
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Rule");
            ui.text_edit_singleline(&mut *rule_str);
            if ui.button("Restart").clicked() {
                let r = rule_str.parse::<Rule>().unwrap();
                *rule = r;
                ev.send(GridReset);
            }
        });
}

fn rotate_g(mut g: Query<&mut Transform, With<MainGrid>>) {
    let Ok(mut g) = g.get_single_mut() else {
        return;
    };
    g.rotate_y(0.02);
}

fn create_grid(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn((
        Grid::new_noise(50),
        MainGrid,
        SpatialBundle::INHERITED_IDENTITY,
        meshes.add(Mesh::from(shape::Cube { size: 0.8 })),
        NoFrustumCulling,
        InstanceMaterialData(vec![]),
    ));
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 9.0, 90.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn update_grid(
    mut g: Query<&mut Grid, With<MainGrid>>,
    rule: Res<Rule>,
    time: Res<Time>,
    mut timer: ResMut<GridTimer>,
    mut task: Local<Option<Task<Grid>>>,
    mut ev: EventReader<GridReset>,
) {
    let Ok(mut g) = g.get_single_mut() else {
        return;
    };
    if ev.read().next().is_some() {
        *g = Grid::new_noise(g.len());
        task.take().map(drop);
    }
    if timer.0.tick(time.delta()).finished() {
        if let Some(next) = task.take().map(block_on) {
            *g = next;
        };
        let _ = task.insert({
            let pool = AsyncComputeTaskPool::get();
            let g = g.clone();
            let rule = rule.clone();
            pool.spawn(async move { g.next(&rule) })
        });
    }
}

fn render_grid_data(mut g: Query<(&mut InstanceMaterialData, &Grid)>, rule: Res<Rule>) {
    for (mut dat, g) in g.iter_mut() {
        *dat = InstanceMaterialData(
            g.iter()
                .filter_map(|(p, c)| {
                    c.is_live().then(|| {
                        let p = Vec3::from(p)
                            - Vec3::new(g.len() as f32, g.len() as f32, g.len() as f32) / 2.;
                        let c = c.color_grad(&rule.states);
                        InstanceData {
                            position: p,
                            scale: 1.,
                            color: c.into(),
                        }
                    })
                })
                .collect(),
        )
    }
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
