use bevy::{prelude::*, window::PresentMode};
use rand::prelude::*;

mod algo;
mod grid;

extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub const CLEAR: Color = Color::rgb(1.0, 1.0, 1.0);

fn initial_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}


#[cfg(target_arch = "wasm32")]
fn main() {
    println!("wasm");
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            title: "Sorting Visualizer".to_string(),
            present_mode: PresentMode::Fifo,
            ..default()
        })
        // PLUGINS
        .add_plugins(DefaultPlugins)
        .add_plugin(grid::Plugin)
        .add_plugin(algo::Plugin)
        // SYSTEMS
        .add_startup_system(initial_setup)
        .add_plugin(bevy_web_resizer::Plugin)
        .run();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("not wasm");
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            title: "Sorting Visualizer".to_string(),
            present_mode: PresentMode::Fifo,
            ..default()
        })
        // PLUGINS
        .add_plugins(DefaultPlugins)
        .add_plugin(grid::Plugin)
        .add_plugin(algo::Plugin)
        // SYSTEMS
        .add_startup_system(initial_setup)
        .run();
}
