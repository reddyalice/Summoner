use crate::prelude::*;
use bevy::utils::hashbrown::HashMap;

#[derive(Resource)]
pub struct Grids {
    grids: HashMap<(u8, u8), Entity>,
    max_x: u8,
    max_y: u8,
    pos_to_layer: HashMap<(u8, u8), u8>,
    layer_to_pos: HashMap<u8, Vec<(u8, u8)>>,
    added_to_layer: Vec<(u8, u8, u8)>,
    removed_from_layer: Vec<(u8, u8, u8)>,
    is_created: bool,
    is_added: bool,
    is_removed: bool,
}

impl Grids {
    pub fn create(
        &mut self,
        grid_map: &GridMap,
        grid_assets: Res<GridRenderAssets>,
        commands: &mut Commands,
    ) {
        if !self.grids.is_empty() {
            self.destroy_all(commands);
        }

        (self.max_x, self.max_y) = grid_map.get_max();
        for (pos, grid_type) in grid_map.get_map().iter() {
            let grid = commands
                .spawn(GridBundle::create(
                    pos.0,
                    pos.1,
                    self.max_x,
                    self.max_y,
                    grid_type,
                    true,
                    grid_assets.grid_mesh.clone(),
                    grid_assets.no_minion_grid_mat[0].clone(),
                    grid_assets.unpassable_grid_mat[0].clone(),
                ))
                .with_children(|parent| {
                    parent.spawn(UnpassBundle::create(
                        grid_assets.unpass_mesh.clone(),
                        grid_assets.unpass_mat.clone(),
                        grid_type,
                        true,
                    ));
                })
                .id();
            self.grids.insert((pos.0, pos.1), grid);
            if *grid_type == GridType::Unpassable {
                self.add_to_or_change_layer(0, pos.0, pos.1);
            }
        }
    }

    pub fn add(
        &mut self,
        x: u8,
        y: u8,
        grid_type: &GridType,
        grid_assets: Res<GridRenderAssets>,
        commands: &mut Commands,
    ) {
        if self.grids.contains_key(&(x, y)) {
            return;
        }
        self.max_x = u8::max(self.max_x, x);
        self.max_y = u8::max(self.max_y, y);

        let grid = commands
            .spawn(GridBundle::create(
                x,
                y,
                self.max_x,
                self.max_y,
                grid_type,
                true,
                grid_assets.grid_mesh.clone(),
                grid_assets.no_minion_grid_mat[0].clone(),
                grid_assets.unpassable_grid_mat[0].clone(),
            ))
            .with_children(|parent| {
                parent.spawn(UnpassBundle::create(
                    grid_assets.unpass_mesh.clone(),
                    grid_assets.unpass_mat.clone(),
                    grid_type,
                    true,
                ));
            })
            .id();
        self.grids.insert((x, y), grid);
        if *grid_type == GridType::Unpassable {
            self.add_to_or_change_layer(0, x, y);
        }
    }

    pub fn add_to_or_change_layer(&mut self, layer: u8, x: u8, y: u8) {
        match self.pos_to_layer.get(&(x, y)) {
            Some(&lay) => {
                if lay == layer {
                    return;
                }
                match self.layer_to_pos.get_mut(&lay) {
                    Some(pos) => match pos.iter().position(|&k| k == (x, y)) {
                        Some(i) => {
                            pos.remove(i);
                        }
                        None => {}
                    },
                    None => {}
                }
            }
            None => {}
        }

        self.pos_to_layer.insert((x, y), layer);
        match self.layer_to_pos.get_mut(&layer) {
            Some(pos) => {
                if !pos.contains(&(x, y)) {
                    pos.push((x, y));
                }
            }
            None => {
                self.layer_to_pos.insert(layer, vec![(x, y)]);
            }
        }

        if self.is_created {
            self.added_to_layer.push((layer, x, y));
            self.is_added = true;
        }
    }

    pub fn remove_from_layer(&mut self, x: u8, y: u8) {
        match self.pos_to_layer.get(&(x, y)) {
            Some(layer) => {
                match self.layer_to_pos.get_mut(layer) {
                    Some(pos) => match pos.iter().position(|&k| k == (x, y)) {
                        Some(i) => {
                            pos.remove(i);
                        }
                        None => {}
                    },
                    None => {}
                }
                if self.is_created {
                    self.removed_from_layer.push((*layer, x, y));
                    self.is_removed = true;
                }
                self.pos_to_layer.remove(&(x, y));
            }
            None => {}
        }
    }

    pub fn destroy(&mut self, x: u8, y: u8, commands: &mut Commands) {
        match self.grids.get(&(x, y)) {
            Some(&grid) => {
                commands.entity(grid).despawn_recursive();
                self.grids.remove(&(x, y));
                self.remove_from_layer(x, y);
            }
            None => {}
        }
    }

    pub fn destroy_all(&mut self, commands: &mut Commands) {
        for (_pos, &grid) in self.grids.iter() {
            commands.entity(grid).despawn_recursive();
        }
        self.grids.clear();
        self.max_x = 0;
        self.max_y = 0;
        self.pos_to_layer.clear();
        self.layer_to_pos.clear();
        self.added_to_layer.clear();
        self.removed_from_layer.clear();
        self.is_created = false;
        self.is_added = false;
        self.is_removed = false;
    }

    pub fn need_update(&self) -> bool {
        self.is_added || self.is_removed || !self.is_created
    }

    pub fn get_grid(&self, x: u8, y: u8) -> Option<&Entity> {
        self.grids.get(&(x, y))
    }

    pub fn get_grids_from_layer(&self, layer: u8) -> Option<Vec<Entity>> {
        match self.layer_to_pos.get(&layer) {
            Some(pos_v) => {
                let mut grid_v = Vec::default();
                for pos in pos_v {
                    match self.grids.get(pos) {
                        Some(&grid) => grid_v.push(grid),
                        None => {}
                    }
                }
                Some(grid_v)
            }
            None => None,
        }
    }

    pub fn get_grids_from_same_layer(&self, x: u8, y: u8) -> Option<Vec<Entity>> {
        match self.pos_to_layer.get(&(x, y)) {
            Some(&layer) => self.get_grids_from_layer(layer),
            None => None,
        }
    }

    pub fn update_layers(
        &mut self,
        mut color_shape_and_rotation: Query<(&mut GridColorAndShape, &mut GridTargetRot)>,
    ) {
        if !self.is_created {
            for (_layer, pos_v) in self.layer_to_pos.iter() {
                for &(x, y) in pos_v.iter() {
                    match self.get_grid(x, y) {
                        Some(&grid) => {
                            if let Ok((mut color_and_shape, mut target_rot)) =
                                color_shape_and_rotation.get_mut(grid)
                            {
                                let up = pos_v.contains(&(x, y + 1)) as u8;
                                let down = if y > 0 {
                                    pos_v.contains(&(x, y - 1)) as u8
                                } else {
                                    0
                                };
                                let right = pos_v.contains(&(x + 1, y)) as u8;
                                let left = if x > 0 {
                                    pos_v.contains(&(x - 1, y)) as u8
                                } else {
                                    0
                                };
                                let result: u8 = up + (down << 1) + (right << 2) + (left << 3);

                                (color_and_shape.shape, target_rot.target_rot) =
                                    compute_shape_and_rot(result);
                            }
                        }
                        None => continue,
                    }
                }
            }
            self.is_created = true;
            return;
        }

        if self.is_added {
            for (layer, x, y) in self.added_to_layer.iter() {
                match self.get_grid(*x, *y) {
                    Some(&grid) => {
                        let mut result = 0;
                        match self.layer_to_pos.get(layer) {
                            Some(pos_v) => {
                                for i in 0..4 {
                                    let mut alt_key = (*x, *y);
                                    match i {
                                        0 => {
                                            alt_key.1 += 1;
                                        }
                                        1 => {
                                            if alt_key.1 == 0 {
                                                continue;
                                            }
                                            alt_key.1 -= 1;
                                        }
                                        2 => {
                                            alt_key.0 += 1;
                                        }
                                        3 => {
                                            if alt_key.0 == 0 {
                                                continue;
                                            }
                                            alt_key.0 -= 1;
                                        }
                                        _ => break,
                                    }

                                    if pos_v.contains(&alt_key) {
                                        match self.get_grid(alt_key.0, alt_key.1) {
                                            Some(&alt_grid) => {
                                                if let Ok((mut color_and_shape, mut target_rot)) =
                                                    color_shape_and_rotation.get_mut(alt_grid)
                                                {
                                                    let up = pos_v
                                                        .contains(&(alt_key.0, alt_key.1 + 1))
                                                        as u8;
                                                    let down = if alt_key.1 > 0 {
                                                        pos_v.contains(&(alt_key.0, alt_key.1 - 1))
                                                            as u8
                                                    } else {
                                                        0
                                                    };
                                                    let right = pos_v
                                                        .contains(&(alt_key.0 + 1, alt_key.1))
                                                        as u8;
                                                    let left = if alt_key.0 > 0 {
                                                        pos_v.contains(&(alt_key.0 - 1, alt_key.1))
                                                            as u8
                                                    } else {
                                                        0
                                                    };

                                                    (
                                                        color_and_shape.shape,
                                                        target_rot.target_rot,
                                                    ) = compute_shape_and_rot(
                                                        up + (down << 1)
                                                            + (right << 2)
                                                            + (left << 3),
                                                    );
                                                    result += 1 << i;
                                                }
                                            }
                                            None => continue,
                                        }
                                    }
                                }
                            }
                            None => {}
                        }

                        if let Ok((mut color_and_shape, mut target_rot)) =
                            color_shape_and_rotation.get_mut(grid)
                        {
                            (color_and_shape.shape, target_rot.target_rot) =
                                compute_shape_and_rot(result);
                        }
                    }
                    None => continue,
                }
            }
            self.added_to_layer.clear();
            self.is_added = false;
        }

        if self.is_removed {
            for (layer, x, y) in self.removed_from_layer.iter() {
                match self.get_grid(*x, *y) {
                    Some(&grid) => {
                        match self.layer_to_pos.get(layer) {
                            Some(pos_v) => {
                                for i in 0..4 {
                                    let mut alt_key = (*x, *y);
                                    match i {
                                        0 => {
                                            alt_key.1 += 1;
                                        }
                                        1 => {
                                            if alt_key.1 == 0 {
                                                continue;
                                            }
                                            alt_key.1 -= 1;
                                        }
                                        2 => {
                                            alt_key.0 += 1;
                                        }
                                        3 => {
                                            if alt_key.0 == 0 {
                                                continue;
                                            }
                                            alt_key.0 -= 1;
                                        }
                                        _ => break,
                                    }

                                    if pos_v.contains(&alt_key) {
                                        match self.get_grid(alt_key.0, alt_key.1) {
                                            Some(&alt_grid) => {
                                                if let Ok((mut color_and_shape, mut target_rot)) =
                                                    color_shape_and_rotation.get_mut(alt_grid)
                                                {
                                                    let up = pos_v
                                                        .contains(&(alt_key.0, alt_key.1 + 1))
                                                        as u8;
                                                    let down = if alt_key.1 > 0 {
                                                        pos_v.contains(&(alt_key.0, alt_key.1 - 1))
                                                            as u8
                                                    } else {
                                                        0
                                                    };
                                                    let right = pos_v
                                                        .contains(&(alt_key.0 + 1, alt_key.1))
                                                        as u8;
                                                    let left = if alt_key.0 > 0 {
                                                        pos_v.contains(&(alt_key.0 - 1, alt_key.1))
                                                            as u8
                                                    } else {
                                                        0
                                                    };

                                                    (
                                                        color_and_shape.shape,
                                                        target_rot.target_rot,
                                                    ) = compute_shape_and_rot(
                                                        up + (down << 1)
                                                            + (right << 2)
                                                            + (left << 3),
                                                    );
                                                }
                                            }
                                            None => continue,
                                        }
                                    }
                                }
                            }
                            None => {}
                        }

                        if let Ok((mut color_and_shape, mut target_rot)) =
                            color_shape_and_rotation.get_mut(grid)
                        {
                            color_and_shape.shape = GridShape::Closed;
                            target_rot.target_rot =
                                Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, 0.0);
                        }
                    }
                    None => continue,
                }
            }
            self.removed_from_layer.clear();
            self.is_removed = false;
        }
    }
}

impl Default for Grids {
    fn default() -> Self {
        Self {
            grids: HashMap::default(),
            max_x: 0,
            max_y: 0,
            pos_to_layer: HashMap::default(),
            layer_to_pos: HashMap::default(),
            added_to_layer: Vec::default(),
            removed_from_layer: Vec::default(),
            is_created: false,
            is_added: false,
            is_removed: false,
        }
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
