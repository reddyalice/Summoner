use crate::prelude::*;

#[derive(Component)]
pub struct CameraZoom {
    zoom : f32,
    initial_zoom : f32
}