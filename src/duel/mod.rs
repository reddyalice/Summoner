use crate::prelude::*;

mod resources;
mod input_systems;
mod update_systems;
mod components;
mod events;

pub use resources::*;
pub use components::*;
pub use events::*;

use input_systems::*;
use update_systems::*;


pub struct DuelPlugin;
impl Plugin for DuelPlugin{
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GridSize>()
            .init_resource::<GridLift>()
            .init_resource::<MinionOffset>()
            .init_resource::<Selection>()
            .init_resource::<GridColorSet>()
            .init_resource::<CombinedGrids>()
            .add_collection_to_loading_state::<_, GridTextureAssets>(GameState::Loading)
            .add_event::<MouseOnGrid>()
            .add_event::<MouseDownGrid>()
            .add_event::<MouseOffGrid>()
            .add_system(setup_duel.in_schedule(OnExit(GameState::Loading)))
            .add_system(spawn_grids.in_schedule(OnEnter(GameState::Duel)))
            .add_system(despawn_grids.in_schedule(OnExit(GameState::Duel)))
            .add_systems((
                update_color_and_shape,
                update_grid_transform,
                update_selection,
                update_mat_set.run_if(resource_exists_and_changed::<GridColorSet>()),
                update_combined_grids.run_if(resource_exists_and_changed::<CombinedGrids>()),      
                mouse_combine_change.run_if(on_event::<MouseDownGrid>()),
                mouse_select_grid.after(mouse_combine_change).run_if(on_event::<MouseDownGrid>()),
                mouse_off_grid.run_if(on_event::<MouseOffGrid>()),
                mouse_on_grid.after(mouse_off_grid).run_if(on_event::<MouseOnGrid>())
            ).in_set(OnUpdate(GameState::Duel)));


    }
}


fn setup_duel(
    mut commands: Commands,
    grid_texture_assets: Res<GridTextureAssets>,
    grid_color_set: Res<GridColorSet>,
    materials: ResMut<Assets<StandardMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    commands.insert_resource(GridRenderAssets::create(
        grid_texture_assets,
        grid_color_set,
        materials,
        meshes,
    ));
}

fn spawn_grids(
    mut commands: Commands,
    grid_assets: Res<GridRenderAssets>,
    grid_lift: Res<GridLift>,
    grid_size: Res<GridSize>,
    mut combined_grids: ResMut<CombinedGrids>,
) {
    let mut rng = rand::thread_rng();

    for y in 0..grid_size.y {
        for x in 0..grid_size.x {
            let passable = rng.gen::<f32>() > 0.2;
            let g = commands
                .spawn(GridBundle::create(
                    x,
                    y,
                    grid_size.x,
                    grid_size.y,
                    grid_lift.lift_distance,
                    passable,
                    true,
                    grid_assets.grid_mesh.clone(),
                    grid_assets.no_minion_grid_mat[0].clone(),
                    grid_assets.unpassable_grid_mat[0].clone(),
                ))
                .with_children(|parent| {
                    parent.spawn(UnpassBundle::create(
                        grid_assets.unpass_mesh.clone(),
                        grid_assets.unpass_mat.clone(),
                        passable,
                        true,
                    ));
                })
                .id();

            if !passable {
                combined_grids.add(0, x, y, g);
            }
        }
    }
}

fn despawn_grids(
    mut commands: Commands,
    grids: Query<Entity, With<GridPos>>,
    mut combined_grids: ResMut<CombinedGrids>,
) {
    for grid in grids.iter() {
        commands.entity(grid).despawn_recursive();
    }
    combined_grids.clear();
}
