use crate::prelude::*;

mod resources;
mod systems;
mod components;
mod events;

pub use resources::*;
pub use components::*;
pub use events::*;
use systems::*;

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
                update_grid_pos,
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
