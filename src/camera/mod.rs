
use crate::prelude::*;

mod resources;
mod systems;
mod components;

pub use resources::*;
pub use components::*;
use systems::*;
use bevy::core_pipeline::bloom::BloomSettings;


pub struct CameraPlugin;
impl  Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(
                DefaultPickingPlugins
                .build()
                .disable::<DefaultHighlightingPlugin>()
                .disable::<DebugPickingPlugin>())
            .add_startup_system(setup_camera);
    }
}

pub fn setup_camera(
    mut commands : Commands
){
    commands.spawn((
        Camera3dBundle{
            camera : Camera { 
                hdr: true,
                ..default()
            },
            transform : Transform { 
            translation: Vec3 { x: 0.0, y: 12.0, z: 12.0 },
            rotation : Quat::from_axis_angle(Vec3::X, -PI/4.0),
            ..default() },
            ..default()
        },
        BloomSettings {
            intensity: 0.1,
            ..default()
        },
        RaycastPickCamera::default()
    ));
}