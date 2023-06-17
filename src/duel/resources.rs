use crate::prelude::*;
use bevy::{render::render_resource::Face, utils::HashMap};

#[derive(Resource)]
pub struct CombinedGrids {
    added_ones: HashMap<(u8, u8, u8), Entity>,
    removed_ones: HashMap<(u8, u8, u8), Entity>,
    inds: HashMap<(u8, u8, u8), Entity>,
    pos_to_layer: HashMap<(u8, u8), u8>,
    layer_to_keys: HashMap<u8, Vec<(u8, u8, u8)>>,
    added: bool,
    removed: bool,
    created: bool,
}

impl CombinedGrids {
    pub fn add(&mut self, layer: u8, x: u8, y: u8, grid: Entity) {
        let key = (layer, x, y);

        match self.pos_to_layer.get(&(x, y)) {
            Some(&l) => {
                if l == layer {
                    return;
                }

                match self.layer_to_keys.get_mut(&l) {
                    Some(k) => match k.iter().position(|&x| x == key) {
                        Some(index) => {
                            k.remove(index);
                        }
                        None => {}
                    },
                    None => {}
                }
                self.inds.remove(&(l, x, y));
            }
            None => {}
        }

        self.pos_to_layer.insert((x, y), layer);
        match self.layer_to_keys.get_mut(&layer) {
            Some(k) => {
                k.push(key);
            }
            None => {
                self.layer_to_keys.insert(layer, vec![key]);
            }
        }
        self.inds.insert(key, grid);

        if self.created {
            self.added_ones.insert(key, grid);
            self.added = true;
        }
    }

    pub fn remove(&mut self, x: u8, y: u8) {
        match self.pos_to_layer.get(&(x, y)) {
            Some(&layer) => {
                let key = (layer, x, y);
                match self.layer_to_keys.get_mut(&layer) {
                    Some(k) => match k.iter().position(|&x| x == key) {
                        Some(index) => {
                            k.remove(index);
                        }
                        None => {}
                    },
                    None => {}
                }

                match self.inds.get(&key) {
                    Some(&grid) => {
                        if self.created {
                            self.removed_ones.insert(key, grid);
                            self.removed = true;
                        }
                        self.inds.remove(&key);
                    }
                    None => {}
                }
                self.pos_to_layer.remove(&(x, y));
            }
            None => {}
        }
    }

    pub fn get_combined_from_layer(&self, layer: u8) -> Option<Vec<Entity>> {
        match self.layer_to_keys.get(&layer) {
            Some(k) => {
                let mut v = Vec::new();
                for key in k {
                    match self.inds.get(key) {
                        Some(&ent) => {
                            v.push(ent);
                        }
                        None => {}
                    }
                }
                Some(v)
            }
            None => None,
        }
    }

    pub fn get_combined_from_pos(&self, x: u8, y: u8) -> Option<Vec<Entity>> {
        match self.pos_to_layer.get(&(x, y)) {
            Some(&layer) => self.get_combined_from_layer(layer),
            None => None,
        }
    }

    pub fn is_combined (&self, x : u8, y : u8) -> bool {
        self.pos_to_layer.contains_key(&(x,y))
    }

    pub fn update(&mut self, mut grids: Query<(&mut GridColorAndShape, &mut GridTargetRot)>) {
        if !self.created {
            for (key, grid) in self.inds.iter() {
                if let Ok((mut color_and_shape, mut target_rot)) = grids.get_mut(*grid) {
                    let up = self.inds.contains_key(&(key.0, key.1, key.2 + 1)) as u8;
                    let down = if key.2 > 0 {
                        self.inds.contains_key(&(key.0, key.1, key.2 - 1)) as u8
                    } else {
                        0
                    };
                    let right = self.inds.contains_key(&(key.0, key.1 + 1, key.2)) as u8;
                    let left = if key.1 > 0 {
                        self.inds.contains_key(&(key.0, key.1 - 1, key.2)) as u8
                    } else {
                        0
                    };
                    let result: u8 = up + (down << 1) + (right << 2) + (left << 3);
                    (color_and_shape.shape, target_rot.target_rot) = compute_shape_and_rot(result);
                }
            }
            self.created = true;
        }

        if self.added {
            for (key, grid) in self.added_ones.iter() {
                let mut result = 0;
                for i in 0..4 {
                    let alt_key;
                    match i {
                        0 => alt_key = (key.0, key.1, key.2 + 1),
                        1 => {
                            if key.2 > 0 {
                                alt_key = (key.0, key.1, key.2 - 1)
                            } else {
                                continue;
                            }
                        }
                        2 => alt_key = (key.0, key.1 + 1, key.2),
                        3 => {
                            if key.1 > 0 {
                                alt_key = (key.0, key.1 - 1, key.2)
                            } else {
                                continue;
                            }
                        }
                        _ => continue,
                    }

                    match self.inds.get(&alt_key) {
                        Some(ng) => {
                            result += 1 << i;
                            if !self.added_ones.contains_key(&alt_key) {
                                let up =
                                    self.inds
                                        .contains_key(&(alt_key.0, alt_key.1, alt_key.2 + 1))
                                        as u8;
                                let down = if alt_key.2 > 0 {
                                    self.inds
                                        .contains_key(&(alt_key.0, alt_key.1, alt_key.2 - 1))
                                        as u8
                                } else {
                                    0
                                };
                                let right =
                                    self.inds
                                        .contains_key(&(alt_key.0, alt_key.1 + 1, alt_key.2))
                                        as u8;
                                let left = if alt_key.1 > 0 {
                                    self.inds
                                        .contains_key(&(alt_key.0, alt_key.1 - 1, alt_key.2))
                                        as u8
                                } else {
                                    0
                                };
                                if let Ok((mut color_and_shape, mut target_rot)) =
                                    grids.get_mut(*ng)
                                {
                                    (color_and_shape.shape, target_rot.target_rot) =
                                        compute_shape_and_rot(
                                            up + (down << 1) + (right << 2) + (left << 3),
                                        );
                                }
                            }
                        }
                        None => {}
                    }
                }

                if let Ok((mut color_and_shape, mut target_rot)) = grids.get_mut(*grid) {
                    (color_and_shape.shape, target_rot.target_rot) = compute_shape_and_rot(result);
                }
            }
            self.added_ones.clear();
            self.added = false;
        }

        if self.removed {
            for (key, grid) in self.removed_ones.iter() {
                if let Ok((mut color_and_shape, mut target_rot)) = grids.get_mut(*grid) {
                    color_and_shape.shape = GridShape::Closed;
                    target_rot.target_rot = Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, 0.0);

                    for i in 0..4 {
                        let alt_key;
                        match i {
                            0 => alt_key = (key.0, key.1, key.2 + 1),
                            1 => {
                                if key.2 > 0 {
                                    alt_key = (key.0, key.1, key.2 - 1)
                                } else {
                                    continue;
                                }
                            }
                            2 => alt_key = (key.0, key.1 + 1, key.2),
                            3 => {
                                if key.1 > 0 {
                                    alt_key = (key.0, key.1 - 1, key.2)
                                } else {
                                    continue;
                                }
                            }
                            _ => continue,
                        }

                        match self.inds.get(&alt_key) {
                            Some(ng) => {
                                if !self.removed_ones.contains_key(&alt_key) {
                                    let up = self.inds.contains_key(&(
                                        alt_key.0,
                                        alt_key.1,
                                        alt_key.2 + 1,
                                    )) as u8;
                                    let down = if alt_key.2 > 0 {
                                        self.inds.contains_key(&(
                                            alt_key.0,
                                            alt_key.1,
                                            alt_key.2 - 1,
                                        )) as u8
                                    } else {
                                        0
                                    };
                                    let right = self.inds.contains_key(&(
                                        alt_key.0,
                                        alt_key.1 + 1,
                                        alt_key.2,
                                    )) as u8;
                                    let left = if alt_key.1 > 0 {
                                        self.inds.contains_key(&(
                                            alt_key.0,
                                            alt_key.1 - 1,
                                            alt_key.2,
                                        )) as u8
                                    } else {
                                        0
                                    };
                                    if let Ok((mut color_and_shape, mut target_rot)) =
                                        grids.get_mut(*ng)
                                    {
                                        (color_and_shape.shape, target_rot.target_rot) =
                                            compute_shape_and_rot(
                                                up + (down << 1) + (right << 2) + (left << 3),
                                            );
                                    }
                                }
                            }
                            None => {}
                        }
                    }
                }
            }
            self.removed_ones.clear();
            self.removed = false;
        }
    }

    pub fn did_change(&self) -> bool {
        self.added || self.removed || !self.created
    }

    pub fn clear(&mut self) {
        self.added_ones.clear();
        self.removed_ones.clear();
        self.inds.clear();
        self.pos_to_layer.clear();
        self.layer_to_keys.clear();
        self.added = false;
        self.removed = false;
        self.created = false;
    }
}

fn compute_shape_and_rot(result: u8) -> (GridShape, Quat) {
    match result {
        0b0000 => (
            GridShape::Closed,
            Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, 0.0),
        ),
        0b0001 => (
            GridShape::Cup,
            Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, PI / 2.0),
        ),
        0b0010 => (
            GridShape::Cup,
            Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, -PI / 2.0),
        ),
        0b0100 => (
            GridShape::Cup,
            Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, 0.0),
        ),
        0b1000 => (
            GridShape::Cup,
            Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, PI),
        ),
        0b0011 => (
            GridShape::Pipe,
            Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, PI / 2.0),
        ),
        0b1100 => (
            GridShape::Pipe,
            Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, 0.0),
        ),
        0b0101 => (
            GridShape::Corner,
            Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, PI / 2.0),
        ),
        0b1001 => (
            GridShape::Corner,
            Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, PI),
        ),
        0b0110 => (
            GridShape::Corner,
            Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, 0.0),
        ),
        0b1010 => (
            GridShape::Corner,
            Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, -PI / 2.0),
        ),
        0b0111 => (
            GridShape::Side,
            Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, PI / 2.0),
        ),
        0b1011 => (
            GridShape::Side,
            Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, -PI / 2.0),
        ),
        0b1101 => (
            GridShape::Side,
            Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, PI),
        ),
        0b1110 => (
            GridShape::Side,
            Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, 0.0),
        ),
        0b1111 => (
            GridShape::Empty,
            Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, 0.0),
        ),
        _ => (
            GridShape::Closed,
            Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, 0.0),
        ),
    }
}

impl Default for CombinedGrids {
    fn default() -> Self {
        CombinedGrids {
            added_ones: HashMap::new(),
            removed_ones: HashMap::new(),
            inds: HashMap::new(),
            pos_to_layer: HashMap::new(),
            layer_to_keys: HashMap::new(),
            added: false,
            removed: false,
            created: false,
        }
    }
}

#[derive(Resource)]
pub struct Selection {
    pub selected_minion: Entity,
}

impl Default for Selection {
    fn default() -> Self {
        Selection {
            selected_minion: Entity::PLACEHOLDER,
        }
    }
}

#[derive(Resource)]
pub struct MinionOffset {
    pub offset: Vec3,
}

impl Default for MinionOffset {
    fn default() -> Self {
        MinionOffset {
            offset: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        }
    }
}

#[derive(Resource)]
pub struct GridSize {
    pub x: u8,
    pub y: u8,
}

impl Default for GridSize {
    fn default() -> Self {
        GridSize { x: 12, y: 5 }
    }
}

#[derive(Resource)]
pub struct GridLift {
    pub lift_speed: f32,
    pub lift_distance: Vec3,
}

impl Default for GridLift {
    fn default() -> Self {
        GridLift {
            lift_speed: 5.0,
            lift_distance: Vec3 {
                x: 0.0,
                y: 0.5,
                z: 0.0,
            },
        }
    }
}

#[derive(AssetCollection, Resource)]
pub struct GridTextureAssets {
    #[asset(path = "grid/grid.png")]
    pub grid_tex: Handle<Image>,
    #[asset(path = "grid/gridcorner.png")]
    pub grid_corner_tex: Handle<Image>,
    #[asset(path = "grid/gridcup.png")]
    pub grid_cup_tex: Handle<Image>,
    #[asset(path = "grid/gridpipe.png")]
    pub grid_pipe_tex: Handle<Image>,
    #[asset(path = "grid/gridside.png")]
    pub grid_side_tex: Handle<Image>,
    #[asset(path = "grid/empty.png")]
    pub empty_tex: Handle<Image>,
    #[asset(path = "grid/cross.png")]
    pub unpass_tex: Handle<Image>,
}

#[derive(Resource)]
pub struct GridColorSet {
    pub selected_color: Color,
    pub minion_on_color: Color,
    pub no_minion_color: Color,
    pub mouse_on_color: Color,
    pub unpass_color: Color,
}

impl Default for GridColorSet {
    fn default() -> Self {
        GridColorSet {
            selected_color: Color::Rgba {
                red: 1.0,
                green: 0.0,
                blue: 0.3,
                alpha: 0.8,
            },
            minion_on_color: Color::Rgba {
                red: 0.95,
                green: 0.86,
                blue: 0.5,
                alpha: 0.5,
            },
            no_minion_color: Color::Rgba {
                red: 0.5,
                green: 0.5,
                blue: 0.5,
                alpha: 0.5,
            },
            mouse_on_color: Color::Rgba {
                red: 0.56,
                green: 0.0,
                blue: 0.15,
                alpha: 0.8,
            },
            unpass_color: Color::Rgba {
                red: 1.0,
                green: 0.0,
                blue: 0.5,
                alpha: 0.8,
            },
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
