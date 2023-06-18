use crate::prelude::*;

pub fn mouse_on_grid(
    mut mouse_on: EventReader<MouseOnGrid>,
    combined_grids: Res<Grids>,
    grid_pos: Query<&GridPos>,
    mut grids: Query<(&GridPassability, &GridSelected, &mut GridColorAndShape)>,
) {
    for ev in mouse_on.iter() {
        if let Ok(pos) = grid_pos.get(ev.0) {
            match combined_grids.get_grids_from_same_layer(pos.x, pos.y) {
                Some(gridvec) => {
                    for ent in gridvec {
                        if let Ok((passable, selected, mut colpr_and_shape)) = grids.get_mut(ent) {
                            if (passable.grid_type == GridType::Passable) && !selected.selected {
                                colpr_and_shape.color = GridColor::MouseOn;
                            }
                        }
                    }
                }
                None => {
                    if let Ok((passable, selected, mut colpr_and_shape)) = grids.get_mut(ev.0) {
                        if (passable.grid_type == GridType::Passable) && !selected.selected {
                            colpr_and_shape.color = GridColor::MouseOn;
                        }
                    }
                }
            }
        }
    }
}

pub fn mouse_combine_change(
    mut mouse_down: EventReader<MouseDownGrid>,
    input: Res<Input<KeyCode>>,
    mut combined_grids: ResMut<Grids>,
    grid_pos: Query<(&GridPos, &GridPassability)>,
    mut grids: Query<(&mut GridColorAndShape, &GridSelected)>,
) {
    for ev in mouse_down.iter() {
        if let Ok((pos, passable)) = grid_pos.get(ev.0) {
            if passable.grid_type == GridType::Passable {
                if input.pressed(KeyCode::LShift) {
                    match combined_grids.get_grids_from_same_layer(pos.x, pos.y) {
                        Some(grid_v) => {
                            for grid in grid_v {
                                if let Ok((mut color, selection)) = grids.get_mut(grid) {
                                    if !selection.selected {
                                        color.color = GridColor::Default;
                                    }
                                }
                            }
                            combined_grids.remove_from_layer(pos.x, pos.y);
                        }
                        None => {
                            combined_grids.add_to_or_change_layer(1, pos.x, pos.y);
                            for grid in combined_grids.get_grids_from_layer(1).unwrap() {
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
    }
}

pub fn mouse_select_grid(
    mut mouse_down: EventReader<MouseDownGrid>,
    combined_grids: Res<Grids>,
    grid_pos: Query<&GridPos>,
    mut grids: Query<(&mut GridSelected, &GridPassability)>,
) {
    for ev in mouse_down.iter() {
        if let Ok(pos) = grid_pos.get(ev.0) {
            match combined_grids.get_grids_from_same_layer(pos.x, pos.y) {
                Some(gridvec) => {
                    let mut selection = false;
                    if let Ok((mut selected, passable)) = grids.get_mut(ev.0) {
                        if passable.grid_type == GridType::Passable {
                            selected.selected = !selected.selected;
                            selection = selected.selected;
                        }
                    }

                    for ent in gridvec {
                        if let Ok((mut selected, passable)) = grids.get_mut(ent) {
                            if passable.grid_type == GridType::Passable {
                                selected.selected = selection;
                            }
                        }
                    }
                }
                None => {
                    if let Ok((mut selected, passable)) = grids.get_mut(ev.0) {
                        if passable.grid_type == GridType::Passable {
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
    combined_grids: Res<Grids>,
    grid_pos: Query<&GridPos>,
    mut grids: Query<(&GridPassability, &GridSelected, &mut GridColorAndShape)>,
) {
    for ev in mouse_off.iter() {
        if let Ok(pos) = grid_pos.get(ev.0) {
            match combined_grids.get_grids_from_same_layer(pos.x, pos.y) {
                Some(gridvec) => {
                    for ent in gridvec {
                        if let Ok((passable, selected, mut colpr_and_shape)) = grids.get_mut(ent) {
                            if (passable.grid_type == GridType::Passable) && !selected.selected {
                                colpr_and_shape.color = GridColor::Default;
                            }
                        }
                    }
                }
                None => {
                    if let Ok((passable, selected, mut colpr_and_shape)) = grids.get_mut(ev.0) {
                        if (passable.grid_type == GridType::Passable) && !selected.selected {
                            colpr_and_shape.color = GridColor::Default;
                        }
                    }
                }
            }
        }
    }
}
