use bevy::prelude::Component;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct GridCell {
    pub x: usize,
    pub y: usize,
    pub is_alive: bool,
}

impl GridCell {
    pub fn new(x: usize, y: usize, is_alive: bool) -> Self {
        GridCell { x, y, is_alive }
    }
}
