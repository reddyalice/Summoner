use crate::prelude::*;

pub const LIFT_DISTANCE: Vec3 = Vec3 {
    x: 0.0,
    y: 0.5,
    z: 0.0,
};
pub const LIFT_SPEED: f32 = 5.0;
pub const MINION_OFFSET: Vec3 = Vec3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

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
