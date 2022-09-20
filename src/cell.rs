use std::fmt::Display;

use bevy::prelude::Component;

#[derive(Debug, Clone, Copy, Component)]
pub struct Cell {
    pub has_dead_ant: bool,
    pub n_ants: usize,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            has_dead_ant: false,
            n_ants: 0,
        }
    }

    pub fn default() -> Self {
        Self {
            has_dead_ant: false,
            n_ants: 0,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut _cell: usize = 0;
        if self.has_dead_ant {
            _cell = 1;
        } else {
            _cell = 0;
        }
        write!(f, "{}", _cell)
    }
}