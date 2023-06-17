use crate::prelude::*;

#[derive(Component)]
pub struct CameraFocus {
    pub zoom : f32,
    pub focus : Vec3,
    pub inital_dir : Vec3
}