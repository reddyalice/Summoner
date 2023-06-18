use crate::prelude::*;
use bevy::render::render_resource::Face;

#[derive(Resource)]
pub struct GridTextureAssets {
    pub grid_tex: Handle<Image>,
    pub grid_corner_tex: Handle<Image>,
    pub grid_cup_tex: Handle<Image>,
    pub grid_pipe_tex: Handle<Image>,
    pub grid_side_tex: Handle<Image>,
    pub empty_tex: Handle<Image>,
    pub unpass_tex: Handle<Image>,
}

impl Load for GridTextureAssets {
    fn load(asset_server : &Res<AssetServer>) -> Self {
        GridTextureAssets {
            grid_tex : asset_server.load("grid/grid.png"),
            grid_corner_tex : asset_server.load("grid/gridcorner.png"),
            grid_cup_tex : asset_server.load("grid/gridcup.png"),
            grid_pipe_tex : asset_server.load("grid/gridpipe.png"),
            grid_side_tex : asset_server.load("grid/gridside.png"),
            empty_tex : asset_server.load("grid/empty.png"),
            unpass_tex : asset_server.load("grid/empty.png")
        }
    }
}

#[derive(Resource)]
pub struct GridMapTextureAssets {
    pub test_map : Handle<Image>
}

impl Load for GridMapTextureAssets {
    fn load(asset_server : &Res<AssetServer>) -> Self {
        GridMapTextureAssets {
            test_map : asset_server.load("grid/map/test_map.png")
        }
    }
}



#[derive(Resource)]
pub struct GridRenderAssets {
    pub grid_mesh: Handle<Mesh>,
    pub unpass_mesh: Handle<Mesh>,
    pub unpass_mat: Handle<StandardMaterial>,

    pub selected_grid_mat: Vec<Handle<StandardMaterial>>,
    pub minion_on_grid_mat: Vec<Handle<StandardMaterial>>,
    pub no_minion_grid_mat: Vec<Handle<StandardMaterial>>,
    pub mouse_on_grid_mat: Vec<Handle<StandardMaterial>>,
    pub unpassable_grid_mat: Vec<Handle<StandardMaterial>>,

    pub empty_mat: Handle<StandardMaterial>,
}

impl GridRenderAssets {
    pub fn create(
        grid_texure_assets: Res<GridTextureAssets>,
        grid_color_set: Res<GridColorSet>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) -> Self {
        GridRenderAssets {
            grid_mesh: meshes.add(create_quad(2.0, 2.0, Option::None, false)),
            unpass_mesh: meshes.add(create_quad(1.5, 1.5, Option::None, false)),
            unpass_mat: materials.add(create_grid_mat(
                grid_color_set.unpass_color,
                grid_texure_assets.unpass_tex.clone(),
            )),

            selected_grid_mat: vec![
                materials.add(create_grid_mat(
                    grid_color_set.selected_color,
                    grid_texure_assets.grid_tex.clone(),
                )),
                materials.add(create_grid_mat(
                    grid_color_set.selected_color,
                    grid_texure_assets.grid_corner_tex.clone(),
                )),
                materials.add(create_grid_mat(
                    grid_color_set.selected_color,
                    grid_texure_assets.grid_cup_tex.clone(),
                )),
                materials.add(create_grid_mat(
                    grid_color_set.selected_color,
                    grid_texure_assets.grid_pipe_tex.clone(),
                )),
                materials.add(create_grid_mat(
                    grid_color_set.selected_color,
                    grid_texure_assets.grid_side_tex.clone(),
                )),
            ],
            minion_on_grid_mat: vec![
                materials.add(create_grid_mat(
                    grid_color_set.minion_on_color,
                    grid_texure_assets.grid_tex.clone(),
                )),
                materials.add(create_grid_mat(
                    grid_color_set.minion_on_color,
                    grid_texure_assets.grid_corner_tex.clone(),
                )),
                materials.add(create_grid_mat(
                    grid_color_set.minion_on_color,
                    grid_texure_assets.grid_cup_tex.clone(),
                )),
                materials.add(create_grid_mat(
                    grid_color_set.minion_on_color,
                    grid_texure_assets.grid_pipe_tex.clone(),
                )),
                materials.add(create_grid_mat(
                    grid_color_set.minion_on_color,
                    grid_texure_assets.grid_side_tex.clone(),
                )),
            ],
            no_minion_grid_mat: vec![
                materials.add(create_grid_mat(
                    grid_color_set.no_minion_color,
                    grid_texure_assets.grid_tex.clone(),
                )),
                materials.add(create_grid_mat(
                    grid_color_set.no_minion_color,
                    grid_texure_assets.grid_corner_tex.clone(),
                )),
                materials.add(create_grid_mat(
                    grid_color_set.no_minion_color,
                    grid_texure_assets.grid_cup_tex.clone(),
                )),
                materials.add(create_grid_mat(
                    grid_color_set.no_minion_color,
                    grid_texure_assets.grid_pipe_tex.clone(),
                )),
                materials.add(create_grid_mat(
                    grid_color_set.no_minion_color,
                    grid_texure_assets.grid_side_tex.clone(),
                )),
            ],
            mouse_on_grid_mat: vec![
                materials.add(create_grid_mat(
                    grid_color_set.mouse_on_color,
                    grid_texure_assets.grid_tex.clone(),
                )),
                materials.add(create_grid_mat(
                    grid_color_set.mouse_on_color,
                    grid_texure_assets.grid_corner_tex.clone(),
                )),
                materials.add(create_grid_mat(
                    grid_color_set.mouse_on_color,
                    grid_texure_assets.grid_cup_tex.clone(),
                )),
                materials.add(create_grid_mat(
                    grid_color_set.mouse_on_color,
                    grid_texure_assets.grid_pipe_tex.clone(),
                )),
                materials.add(create_grid_mat(
                    grid_color_set.mouse_on_color,
                    grid_texure_assets.grid_side_tex.clone(),
                )),
            ],
            unpassable_grid_mat: vec![
                materials.add(create_grid_mat(
                    grid_color_set.unpass_color,
                    grid_texure_assets.grid_tex.clone(),
                )),
                materials.add(create_grid_mat(
                    grid_color_set.unpass_color,
                    grid_texure_assets.grid_corner_tex.clone(),
                )),
                materials.add(create_grid_mat(
                    grid_color_set.unpass_color,
                    grid_texure_assets.grid_cup_tex.clone(),
                )),
                materials.add(create_grid_mat(
                    grid_color_set.unpass_color,
                    grid_texure_assets.grid_pipe_tex.clone(),
                )),
                materials.add(create_grid_mat(
                    grid_color_set.unpass_color,
                    grid_texure_assets.grid_side_tex.clone(),
                )),
            ],
            empty_mat: materials.add(create_grid_mat(
                Color::Rgba {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 0.0,
                },
                grid_texure_assets.empty_tex.clone(),
            )),
        }
    }
}

fn create_grid_mat(color: Color, image: Handle<Image>) -> StandardMaterial {
    StandardMaterial {
        base_color: color,
        base_color_texture: Some(image),
        cull_mode: Some(Face::Back),
        unlit: true,
        perceptual_roughness: 0.5,
        reflectance: 0.15,
        alpha_mode: AlphaMode::Blend,
        ..default()
    }
}