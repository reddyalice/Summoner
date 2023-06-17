
use crate::prelude::*;
use bevy::core_pipeline::bloom::BloomSettings;


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