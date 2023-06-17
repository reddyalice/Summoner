use crate::prelude::*;
pub struct MouseOnGrid(pub Entity);
impl From<ListenedEvent<Over>> for MouseOnGrid {
    fn from(event: ListenedEvent<Over>) -> Self {
        MouseOnGrid(event.target)
    }
}

pub struct MouseDownGrid(pub Entity);
impl From<ListenedEvent<Down>> for MouseDownGrid {
    fn from(event: ListenedEvent<Down>) -> Self {
        MouseDownGrid(event.target)
    }
}

pub struct MouseOffGrid(pub Entity);
impl From<ListenedEvent<Out>> for MouseOffGrid {
    fn from(event: ListenedEvent<Out>) -> Self {
        MouseOffGrid(event.target)
    }
}