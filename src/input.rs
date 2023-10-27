use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

pub struct NatInputPlugin;

impl Plugin for NatInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, cursor_position);
    }
}


fn cursor_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    // Games typically only have one window (the primary window)
    if let Some(position) = q_windows.single().cursor_position() {
        println!("Cursor is inside the primary window, at {:?}", position);
    } else {
        println!("Cursor is not in the game window.");
    }
}