use crate::prelude::*;

pub fn update_grid_transform(
    mut grids: Query<(&mut Transform, &GridTargetPos, &GridTargetRot)>,
    time: Res<Time>,
) {
    grids
        .par_iter_mut()
        .for_each_mut(|(mut transform, target_pos, target_rot)| {
            if transform.translation != target_pos.target_pos {
                transform.translation = transform.translation.lerp(
                    target_pos.target_pos,
                    time.delta_seconds() * LIFT_SPEED,
                );
            }

            if transform.rotation != target_rot.target_rot {
                transform.rotation = transform.rotation.lerp(
                    target_rot.target_rot,
                    time.delta_seconds() * LIFT_SPEED * 2.0,
                );
            }
        });
}

pub fn update_selection(
    mut grids: Query<
        (
            &GridSelected,
            &GridDefaultPos,
            &GridPassability,
            &mut GridTargetPos,
            &mut GridColorAndShape,
        ),
        Changed<GridSelected>,
    >,
) {
    grids.par_iter_mut().for_each_mut(
        |(selection, default_pos, passable, mut target_pos, mut color_and_shape)| {
            if passable.grid_type == GridType::Passable {
                if selection.selected {
                    color_and_shape.color = GridColor::Selected;
                    target_pos.target_pos = default_pos.default_pos + LIFT_DISTANCE;
                } else {
                    target_pos.target_pos = default_pos.default_pos;
                }
            }
        },
    );
}

pub fn update_layers(
    mut grids: ResMut<Grids>,
    color_shape_and_rot : Query<(&mut GridColorAndShape, &mut GridTargetRot)>,
) {
    if grids.need_update() {
        grids.update_layers(color_shape_and_rot);
    }
}

pub fn update_color_and_shape(
    grid_assets: Res<GridRenderAssets>,
    mut indicators: Query<
        (
            &GridMinion,
            &GridColorAndShape,
            &mut Handle<StandardMaterial>,
        ),
        Changed<GridColorAndShape>,
    >,
) {
    for (minion, color_and_shape, mut material) in indicators.iter_mut() {
        if color_and_shape.shape == GridShape::Empty {
            *material = grid_assets.empty_mat.clone();
            continue;
        }
        let shape = color_and_shape.shape as usize;
        match color_and_shape.color {
            GridColor::Selected => *material = grid_assets.selected_grid_mat[shape].clone(),
            GridColor::Default => {
                if minion.minion == Entity::PLACEHOLDER {
                    *material = grid_assets.no_minion_grid_mat[shape].clone()
                } else {
                    *material = grid_assets.minion_on_grid_mat[shape].clone()
                }
            }
            GridColor::MouseOn => *material = grid_assets.mouse_on_grid_mat[shape].clone(),
            GridColor::Unpassable => *material = grid_assets.unpassable_grid_mat[shape].clone(),
        }
    }
}

pub fn update_mat_set(
    mut materials: ResMut<Assets<StandardMaterial>>,
    grid_color_set: Res<GridColorSet>,
    grid_assets: Res<GridRenderAssets>,
) {
    for i in 0..5 {
        if let Some(mut mat) = materials.get_mut(&grid_assets.minion_on_grid_mat[i]) {
            mat.base_color = grid_color_set.minion_on_color
        }
        if let Some(mut mat) = materials.get_mut(&grid_assets.mouse_on_grid_mat[i]) {
            mat.base_color = grid_color_set.mouse_on_color
        }
        if let Some(mut mat) = materials.get_mut(&grid_assets.no_minion_grid_mat[i]) {
            mat.base_color = grid_color_set.no_minion_color
        }
        if let Some(mut mat) = materials.get_mut(&grid_assets.selected_grid_mat[i]) {
            mat.base_color = grid_color_set.selected_color
        }
        if let Some(mut mat) = materials.get_mut(&grid_assets.unpassable_grid_mat[i]) {
            mat.base_color = grid_color_set.unpass_color
        }
    }
    if let Some(mut mat) = materials.get_mut(&grid_assets.unpass_mat) {
        mat.base_color = grid_color_set.unpass_color
    }
}
