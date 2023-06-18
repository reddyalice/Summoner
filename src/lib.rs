mod camera;
mod duel;
mod load_assets;
mod minion;

pub mod prelude {
    pub use bevy::prelude::*;
    use bevy::render::mesh::Indices;
    use bevy::render::render_resource::PrimitiveTopology;
    pub use bevy_inspector_egui::quick::*;
    pub use bevy_mod_picking::prelude::*;
    pub use rand::prelude::*;

    pub use crate::camera::*;
    pub use crate::duel::*;
    pub use crate::load_assets::*;
    pub use crate::minion::*;
    pub use bevy_mod_picking::*;
    pub use std::f32::consts::PI;

    #[derive(States, PartialEq, Eq, Default, Debug, Clone, Hash, Reflect)]
    pub enum GameState {
        #[default]
        Loading,
        Idle,
        Duel,
    }

    pub fn create_quad(w: f32, h: f32, pivot: Option<Vec2>, double_sided: bool) -> Mesh {
        let w2 = w / 2.0;
        let h2 = h / 2.0;
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let vertices = match pivot {
            None => {
                vec![
                    [-w2, -h2, 0.0],
                    [w2, -h2, 0.0],
                    [-w2, h2, 0.0],
                    [w2, h2, 0.0],
                    [-w2, -h2, 0.0],
                    [w2, -h2, 0.0],
                    [-w2, h2, 0.0],
                    [w2, h2, 0.0],
                ]
            }
            Some(pivot) => {
                let px = pivot.x * w;
                let py = pivot.y * h;
                vec![
                    [-px, -py, 0.0],
                    [w - px, -py, 0.0],
                    [-px, h - py, 0.0],
                    [w - px, h - py, 0.0],
                    [-px, -py, 0.0],
                    [w - px, -py, 0.0],
                    [-px, h - py, 0.0],
                    [w - px, h - py, 0.0],
                ]
            }
        };

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vec![
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
            ],
        );

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![
                [0.0, 1.0],
                [1.0, 1.0],
                [0.0, 0.0],
                [1.0, 0.0],
                [0.0, 1.0],
                [1.0, 1.0],
                [0.0, 0.0],
                [1.0, 0.0],
            ],
        );

        mesh.set_indices(Some(Indices::U32(if double_sided {
            vec![0, 1, 2, 1, 3, 2, 5, 4, 6, 7, 5, 6]
        } else {
            vec![0, 1, 2, 1, 3, 2]
        })));

        mesh
    }

    pub trait Load {
        fn load(asset_server: &Res<AssetServer>) -> Self;
    }
}
