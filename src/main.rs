mod player;
mod camera;
mod world;
mod input;

pub use bevy::{
    prelude::*,
    window::{WindowResizeConstraints, WindowResolution},
};

use player::PlayerPlugin; 
use camera::CameraPlugin;
use world::WorldPlugin;
use input::NatInputPlugin;

mod prelude {
    // constants

    // screen variables
    pub const INITIAL_WIDTH: u32 = 320;
    pub const INITIAL_HEIGHT: u32 = 240;
    pub const SCALE_FACTOR: f32 = 2.0;
}

use prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Spagnum".into(),
                    resolution: WindowResolution::new(
                        INITIAL_WIDTH as f32 * SCALE_FACTOR,
                        INITIAL_HEIGHT as f32 * SCALE_FACTOR,
                    ),
                    resize_constraints: WindowResizeConstraints {
                        min_width: INITIAL_WIDTH as f32 * SCALE_FACTOR,
                        min_height: INITIAL_HEIGHT as f32 * SCALE_FACTOR,
                        ..default()
                    },
                    fit_canvas_to_parent: true,
                    resizable: true,
                    ..default()
                }),
                ..default()
            }), 
            PlayerPlugin,
            CameraPlugin,
            WorldPlugin,
            NatInputPlugin
        ))
        .run();
}
