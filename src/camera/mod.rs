
use crate::prelude::*;

mod resources;
mod systems;
mod components;

pub use resources::*;
pub use components::*;
use systems::*;



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