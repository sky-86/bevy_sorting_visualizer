use bevy::{prelude::*, window::PresentMode};
use rand::prelude::*;
use wasm_bindgen::prelude::*;

mod algo;
mod grid;
mod utils;

pub const CLEAR: Color = Color::rgb(1.0, 1.0, 1.0);

fn initial_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

#[wasm_bindgen]
pub fn main() {
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

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, pack-sort!");
}
