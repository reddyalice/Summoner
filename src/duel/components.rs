use crate::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GridShape{
    Closed  = 0,
    Corner = 1,
    Cup = 2,
    Pipe = 3,
    Side = 4,
    Empty = -1
}


pub enum GridColor {
    Default,
    Selected,
    MouseOn,
    Unpassable
}

#[derive(Component)]
pub struct GridColorAndShape{
    pub shape : GridShape,
    pub color : GridColor
}


#[derive(Component)]
pub struct GridUnpass;

#[derive(Component, PartialEq, Eq, Hash)]
pub struct GridPos {
    pub x: u8,
    pub y: u8,
}

#[derive(Component)]
pub struct GridDefaultPos {
    pub default_pos: Vec3,
}

#[derive(Component)]
pub struct GridTargetPos {
    pub target_pos: Vec3,
}

#[derive(Component)]
pub struct GridTargetRot {
    pub target_rot: Quat,
}

#[derive(Component)]
pub struct GridPassability {
    pub passable: bool,
    pub show_passable: bool,
}

#[derive(Component)]
pub struct GridSelected {
    pub selected: bool,
}

#[derive(Component)]
pub struct GridMinion {
    pub minion : Entity
}


#[derive(Bundle)]
pub struct GridBundle {
   
    #[bundle]
    pbr: PbrBundle,
    grid_pos: GridPos,
    default_pos: GridDefaultPos,
    target_pos: GridTargetPos,
    target_rot : GridTargetRot,
    passability: GridPassability,
    selected: GridSelected,
    color_and_shape : GridColorAndShape,
    minion : GridMinion,
    pick_target: RaycastPickTarget,
    mouse_on: OnPointer<Over>,
    mouse_down: OnPointer<Down>,
    mouse_off: OnPointer<Out>,
    name: Name
}

impl GridBundle {
    pub fn create(
        x: u8,
        y: u8,
        size_x: u8,
        size_y: u8,
        lift_distance: Vec3,
        passable: bool,
        show_passable: bool,
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
        unpass_material: Handle<StandardMaterial>
    ) -> GridBundle {
        let def_pos = Vec3 {
            x: x as f32 * 2.0 - size_x as f32 + 1.0,
            y: 0.0,
            z: -(y as f32 * 2.0 - size_y as f32 + 1.0),
        };
        GridBundle {
            
            pbr: PbrBundle {
                mesh,
                material: if passable { material } else { unpass_material },
                transform: Transform {
                    translation: def_pos,
                    rotation: Quat::from_rotation_x(-PI / 2.0),
                    ..default()
                },
                visibility: if !show_passable && !passable {
                    Visibility::Hidden
                } else {
                    Visibility::Visible
                },
                ..default()
    
            },
            grid_pos: GridPos { x: x, y: y },
            default_pos: GridDefaultPos {
                default_pos: def_pos,
            },
            target_pos: GridTargetPos {
                target_pos: if passable {
                    def_pos
                } else {
                    def_pos - lift_distance
                },
            },
            target_rot : GridTargetRot { target_rot: Quat::from_rotation_x(-PI / 2.0) },
            passability: GridPassability {
                passable,
                show_passable: show_passable,
            },
            color_and_shape: GridColorAndShape { 
                shape: GridShape::Closed, 
                color: if passable {GridColor::Default} else {GridColor::Unpassable} },
            minion : GridMinion { minion: Entity::PLACEHOLDER },
            pick_target: RaycastPickTarget::default(),
            mouse_on: OnPointer::<Over>::send_event::<MouseOnGrid>(),
            mouse_down: OnPointer::<Down>::send_event::<MouseDownGrid>(),
            mouse_off: OnPointer::<Out>::send_event::<MouseOffGrid>(),
            selected: GridSelected { selected: false },
            name: Name::new(format!("Grid [{},{}]", x, y)),
        }
    }
}



#[derive(Bundle)]
pub struct UnpassBundle {
    #[bundle]
    pbr: PbrBundle,
    name: Name,
    tag: GridUnpass,
}

impl UnpassBundle {
    pub fn create(
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
        passable: bool,
    ) -> UnpassBundle {
        UnpassBundle {
            pbr: PbrBundle {
                mesh,
                material,
                visibility: if !passable {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                },
                ..default()
            },
            name: Name::new("Unpass"),
            tag: GridUnpass,
        }
    }
}
