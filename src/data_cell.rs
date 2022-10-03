
use bevy::prelude::Component;
use crate::food::Food;

#[derive(Debug, Clone, Copy, Component)]
pub struct DataCell {
    pub food: Option<Food>
}

impl DataCell {
    pub fn new(food: Option<Food>) -> Self {
        Self {
            food
        }
    }

    pub fn default() -> Self {
        Self {
            food: None
        }
    }
}