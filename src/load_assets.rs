use crate::prelude::*;


pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems((
                load_textures,
                |mut game_state: ResMut<NextState<GameState>>| { game_state.set(GameState::Idle); }
            ).in_set(OnUpdate(GameState::Loading)))
            .add_system(setup_grid_render_assets.in_schedule(OnExit(GameState::Loading)));
    }
}


fn load_textures(
    mut commands : Commands,
    asset_server : Res<AssetServer>){
        commands.insert_resource(GridTextureAssets::load(&asset_server));
        commands.insert_resource(GridMapTextureAssets::load(&asset_server));

}

fn setup_grid_render_assets(
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