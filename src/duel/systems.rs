use crate::prelude::*;

pub fn setup_duel(
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

pub fn spawn_grids(
    mut commands: Commands,
    grid_assets: Res<GridRenderAssets>,
    grid_lift: Res<GridLift>,
    mut grid_size: ResMut<GridSize>,
    mut combined_grids: ResMut<CombinedGrids>,
) {
    let mut rng = rand::thread_rng();

    grid_size.x = rng.gen_range(12..24);
    grid_size.y = rng.gen_range(5..10);

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
                    ));
                })
                .id();

            if !passable {
                combined_grids.add(0, x, y, g);
            }
        }
    }
}

pub fn despawn_grids(
    mut commands: Commands,
    grids: Query<Entity, With<GridPos>>,
    mut combined_grids: ResMut<CombinedGrids>,
) {
    for grid in grids.iter() {
        commands.entity(grid).despawn_recursive();
    }
    combined_grids.clear();
}

pub fn update_grid_pos(
    mut grids: Query<(&mut Transform, &GridTargetPos, &GridTargetRot)>,
    grid_lift: Res<GridLift>,
    time: Res<Time>,
) {
    for (mut transform, target_pos, target_rot) in grids.iter_mut() {
        if transform.translation != target_pos.target_pos {
            transform.translation = transform.translation.lerp(
                target_pos.target_pos,
                time.delta_seconds() * grid_lift.lift_speed,
            );
        }

        if transform.rotation != target_rot.target_rot {
            transform.rotation = transform.rotation.lerp(
                target_rot.target_rot,
                time.delta_seconds() * grid_lift.lift_speed * 2.0,
            );
        }
    }
}

pub fn mouse_on_grid(
    mut mouse_on: EventReader<MouseOnGrid>,
    combined_grids: Res<CombinedGrids>,
    grid_pos: Query<&GridPos>,
    mut grids: Query<(&GridPassability, &GridSelected, &mut GridColorAndShape)>,
) {
    for ev in mouse_on.iter() {
        if let Ok(pos) = grid_pos.get(ev.0) {
            match combined_grids.get_combined_from_pos(pos.x, pos.y) {
                Some(gridvec) => {
                    for ent in gridvec {
                        if let Ok((passable, selected, mut colpr_and_shape)) = grids.get_mut(ent) {
                            if passable.passable && !selected.selected {
                                colpr_and_shape.color = GridColor::MouseOn;
                            }
                        }
                    }
                }
                None => {
                    if let Ok((passable, selected, mut colpr_and_shape)) = grids.get_mut(ev.0) {
                        if passable.passable && !selected.selected {
                            colpr_and_shape.color = GridColor::MouseOn;
                        }
                    }
                }
            }
        }
    }
}


pub fn mouse_combine_change (
    mut mouse_down: EventReader<MouseDownGrid>,
    input: Res<Input<KeyCode>>,
    mut combined_grids: ResMut<CombinedGrids>,
    grid_pos : Query<&GridPos>,
    mut grids : Query<(&mut GridColorAndShape, &GridSelected)> 
){
    for ev in mouse_down.iter() {
        if let Ok(pos) = grid_pos.get(ev.0) {
            if input.pressed(KeyCode::LShift) {
                if combined_grids.is_combined(pos.x, pos.y) {
                    for grid in combined_grids.get_combined_from_pos(pos.x, pos.y).unwrap(){
                        if let Ok((mut color, selection)) = grids.get_mut(grid) {
                            if !selection.selected {
                                color.color = GridColor::Default;
                            }
                        }
                    }
                    combined_grids.remove(pos.x, pos.y);
                } else {
                    combined_grids.add(1, pos.x, pos.y, ev.0);
                    for grid in combined_grids.get_combined_from_layer(1).unwrap(){
                        if let Ok((mut color, selection)) = grids.get_mut(grid) {
                            if !selection.selected {
                                color.color = GridColor::MouseOn;
                            }
                        }
                    }
                }
            }
        }
    }
}



pub fn mouse_select_grid(
    mut mouse_down: EventReader<MouseDownGrid>,
    combined_grids: Res<CombinedGrids>,
    grid_pos: Query<&GridPos>,
    mut grids: Query<(&mut GridSelected, &GridPassability)>,
) {
    for ev in mouse_down.iter() {
        if let Ok(pos) = grid_pos.get(ev.0) {
            match combined_grids.get_combined_from_pos(pos.x, pos.y) {
                Some(gridvec) => {
                    let mut selection = false;
                    if let Ok((mut selected, passability)) = grids.get_mut(ev.0) {
                        if passability.passable {
                            selected.selected = !selected.selected;
                            selection = selected.selected;
                        }
                    }

                    for ent in gridvec {
                        if let Ok((mut selected, passability)) = grids.get_mut(ent) {
                            if passability.passable {
                                selected.selected = selection;
                            }
                        }
                    }
                }
                None => {
                    if let Ok((mut selected, passability)) = grids.get_mut(ev.0) {
                        if passability.passable {
                            selected.selected = !selected.selected;
                        }
                    }
                }
            }
        }
    }
}

pub fn mouse_off_grid(
    mut mouse_off: EventReader<MouseOffGrid>,
    combined_grids: Res<CombinedGrids>,
    grid_pos: Query<&GridPos>,
    mut grids: Query<(&GridPassability, &GridSelected, &mut GridColorAndShape)>,
) {
    for ev in mouse_off.iter() {
        if let Ok(pos) = grid_pos.get(ev.0) {
            match combined_grids.get_combined_from_pos(pos.x, pos.y) {
                Some(gridvec) => {
                    for ent in gridvec {
                        if let Ok((passable, selected, mut colpr_and_shape)) = grids.get_mut(ent) {
                            if passable.passable && !selected.selected {
                                colpr_and_shape.color = GridColor::Default;
                            }
                        }
                    }
                }
                None => {
                    if let Ok((passable, selected, mut colpr_and_shape)) = grids.get_mut(ev.0) {
                        if passable.passable && !selected.selected {
                            colpr_and_shape.color = GridColor::Default;
                        }
                    }
                }
            }
        }
    }
}

pub fn update_selection(
    grid_lift: Res<GridLift>,
    mut grids: Query<
        (
            &GridSelected,
            &GridDefaultPos,
            &mut GridTargetPos,
            &mut GridColorAndShape,
        ),
        Changed<GridSelected>,
    >,
) {
    for (selection, default_pos, mut target_pos, mut color_and_shape) in grids.iter_mut() {
        if selection.selected {
            color_and_shape.color = GridColor::Selected;
            target_pos.target_pos = default_pos.default_pos + grid_lift.lift_distance;
        } else {
            target_pos.target_pos = default_pos.default_pos;
        }
    }
}

pub fn update_combined_grids(
    mut combined_grids: ResMut<CombinedGrids>,
    grids: Query<(&mut GridColorAndShape, &mut GridTargetRot)>,
) {
    if combined_grids.did_change() {
        combined_grids.update(grids);
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
