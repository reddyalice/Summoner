use crate::prelude::*;

#[derive(Component)]
pub struct OnGrids{
    pub grids : Vec<Entity>
}

#[derive(Component)]
pub struct MinionAnimationStep{
    pub step : u8
}

#[derive(Component, Default)]
pub enum MinionAnimationState{
    #[default]
    Idle,
    Walking
}
