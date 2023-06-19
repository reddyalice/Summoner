

use bevy::input::mouse::MouseWheel;

use crate::prelude::*;

pub fn update_cam_position(
    time : Res<Time>,
    mut camera_pos : Query<(&mut Transform, &CameraFocus)>
) {
    for (mut transform, focus) in camera_pos.iter_mut(){
        let new = focus.focus + focus.inital_dir * focus.zoom;
        if transform.translation != new {
            transform.translation = transform.translation.lerp(new, time.delta_seconds() * 10.0);
        }
        
    }
}

pub fn move_camera(
    time : Res<Time>,
    input: Res<Input<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut foci : Query<&mut CameraFocus>
) {
    match foci.get_single_mut(){
        Ok(mut focus) => {
            let mut dir = Vec2{x:0.0, y:0.0};
            if input.pressed(KeyCode::W){
                dir.y -= 1.0;
            }
            if input.pressed(KeyCode::S){
                dir.y += 1.0;
            }
            if input.pressed(KeyCode::D){
                dir.x += 1.0;
            }
            if input.pressed(KeyCode::A){
                dir.x -= 1.0;
            }
            dir = dir.normalize_or_zero();
            focus.focus += Vec3{ x: dir.x, y:0.0, z: dir.y} * time.delta_seconds() * 15.0;
            for ev in scroll_evr.iter(){
                focus.zoom -= ev.y * time.delta_seconds() * 10.0;
            }
        },
        Err(_) => {},
    }



}





