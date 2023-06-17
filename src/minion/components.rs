use crate::prelude::*;

#[derive(Component)]
pub struct OnGrids{
    pub grids : Vec<Entity>
}

#[derive(Component)]
pub struct MinionAnimationStep{
    pub step : u8,
    pub state : MinionAnimationState
}


pub enum MinionAnimationState{
    Idle,
    Walking
}
