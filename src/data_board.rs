use crate::data_cell::DataCell;
use crate::food::Food;
use crate::config::Config;
use bevy::prelude::*;
use rand::distributions::{Uniform, Distribution};

use std::{fs};


#[derive(Debug, Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub content: Vec<Vec<Entity>>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        let content = vec![vec![Entity::from_raw(0); size]; size];

        Self {
            width: size,
            height: size,
            content,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new(50)
    }
}

pub fn read_data_from_base(base: &String) -> Vec<Food> {
    let base_path = format!("base/{base_arg}", base_arg = base);
    let content = fs::read_to_string(base_path).expect("Could not read the file");

    let mut data: Vec<Food> = Vec::new();
    
    for line in content.split("\n") {
        let line = line.trim();
        if line.is_empty() || line.chars().nth(0) == Some('#') {
            continue;
        }

        let line = line.replace(',', ".");
        let values: Vec<&str> = line.split_whitespace().collect();
        let (d1, d2, label) = (values[0].parse::<f32>().unwrap(), values[1].parse::<f32>().unwrap(), values[2].parse::<usize>().unwrap());

        data.push(Food{d1, d2, label});
    }

    data
}

pub fn setup_board(mut commands: Commands, windows: Res<Windows>, mut board: ResMut<Board>, params: ResMut<Config>) {
    let window = windows.primary();
    
    let border_width = params.border_size;
    let cell_width =
        (window.width() - border_width * (board.width - 1) as f32) / (board.width as f32);
    let cell_height =
        (window.height() - border_width * (board.height - 1) as f32) / (board.height as f32);
    for xx in 0..board.width {
        for yy in 0..board.height {
            let x = xx as f32;
            let y = yy as f32;
            let cx = -window.width() / 2. + cell_width * x + border_width * x + cell_width / 2.;
            let cy = -window.height() / 2. + cell_height * y + border_width * y + cell_height / 2.;
            let entity = commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform::from_xyz(cx, cy, 1.0),
                    sprite: Sprite {
                        color: Color::rgb(1., 1., 1.),
                        custom_size: Some(Vec2::new(cell_width, cell_height)),
                        ..default()
                    },
                    ..default()
                })
                .insert(DataCell::default())
                .id();
            board.content[xx][yy] = entity;
        }
    }
}

pub fn setup_dead_ants(board: Res<Board>, mut query: Query<&mut DataCell>, params: Res<Config>) {

    let foods = read_data_from_base(&params.base);

    let between_width = Uniform::from(0..board.width);
    let between_height = Uniform::from(0..board.height);
    let mut rng = rand::thread_rng();

    for index in 0..foods.len() {

        loop {
            let new_x = between_width.sample(&mut rng);
            let new_y = between_height.sample(&mut rng);
            let mut cell = query.get_mut(board.content[new_x][new_y]).unwrap();

            match cell.food {
                Some(_) => {},
                None => {
                    cell.food = Some(foods[index]);
                    break;
                }
            }
        }

    }
}

fn get_color_from_label(food: Option<Food>) -> Color {
    match food {
        Some(food) => {
            let x = food.label as f32;

            let r = 1. % x;
            let g = 1. -  1. * (x / 10.).min(1.) % 1.;
            let b = 1. * (x / 10.).min(1.);

            Color::rgb(r, g, b)
        },
        None => Color::rgb(1.0, 1.0, 1.0),
        // None => Color::rgb(0.9, 0.45, 0.0),
    }
}

pub fn color_cells(mut query_cell: Query<(&DataCell, &mut Sprite), Changed<DataCell>>) {
    for (cell, mut sprite) in query_cell.iter_mut() {
        sprite.color = get_color_from_label(cell.food);
    }
}
