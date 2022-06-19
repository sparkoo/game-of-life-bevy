use bevy::prelude::*;

use crate::consts;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Cell {
    pub state: CellState,
}

impl Cell {
    pub fn change(&mut self, new_state: CellState) {
        self.state = new_state;
    }
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq)]
pub enum CellState {
    Alive,
    Dead,
}

impl CellState {
    pub fn color(&self) -> Color {
        match *self {
            CellState::Alive => consts::CELL_ALIVE_COLOR,
            CellState::Dead => consts::CELL_DEAD_COLOR,
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
