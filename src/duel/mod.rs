use crate::prelude::*;

mod asset_handles;
mod grid_map;
mod grids_resource;
mod resources;
mod input_systems;
mod update_systems;
mod components;
mod events;

pub use asset_handles::*;
pub use grid_map::*;
pub use grids_resource::*;
pub use resources::*;
pub use components::*;
pub use events::*;


use input_systems::*;
use update_systems::*;


pub struct DuelPlugin;
impl Plugin for DuelPlugin{
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Grids>()
            .init_resource::<Selection>()
            .init_resource::<GridColorSet>()
            .add_event::<MouseOnGrid>()
            .add_event::<MouseDownGrid>()
            .add_event::<MouseOffGrid>()
            .add_system(spawn_grids.in_schedule(OnEnter(GameState::Duel)))
            .add_system(despawn_grids.in_schedule(OnExit(GameState::Duel)))
            .add_systems((
                update_color_and_shape,
                update_grid_transform,
                update_selection,
                update_mat_set.run_if(resource_exists_and_changed::<GridColorSet>()),
                update_layers.run_if(resource_exists_and_changed::<Grids>()),      
                mouse_combine_change.run_if(on_event::<MouseDownGrid>()),
                mouse_select_grid.after(mouse_combine_change).run_if(on_event::<MouseDownGrid>()),
                mouse_off_grid.run_if(on_event::<MouseOffGrid>()),
                mouse_on_grid.after(mouse_off_grid).run_if(on_event::<MouseOnGrid>())
            ).in_set(OnUpdate(GameState::Duel)));


    }
}




fn spawn_grids(
    mut commands: Commands,
    grid_assets: Res<GridRenderAssets>,
    mut grids : ResMut<Grids>,
    images : Res<Assets<Image>>,
    map_textures : Res<GridMapTextureAssets>
) {
   
    let grid_map =  GridMap::create_from_image(images.get(&map_textures.test_map).unwrap());
    /*for x in 0..12 as u8 {
        for y in 0..6 as u8 {
            grid_map.add(x, y, GridType::Passable);
        }
    }*/


    grids.create(&grid_map, grid_assets, &mut commands);
    
}

fn despawn_grids(
    mut commands: Commands,
    mut grids : ResMut<Grids>
) {
    grids.destroy_all(&mut commands);
}
