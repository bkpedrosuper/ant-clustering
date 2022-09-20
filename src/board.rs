use crate::cell::Cell;
use crate::config::Config;
use bevy::prelude::*;
use rand::distributions::{Uniform, Distribution};

#[derive(Debug, Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub content: Vec<Vec<Entity>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let content = vec![vec![Entity::from_raw(0); width]; height];

        Self {
            width,
            height,
            content,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new(50, 50)
    }
}


pub fn setup_board(mut commands: Commands, windows: Res<Windows>, mut board: ResMut<Board>, mut params: ResMut<Config>) {
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
                .insert(Cell::default())
                .id();
            board.content[xx][yy] = entity;
        }
    }
}

pub fn setup_dead_ants(board: Res<Board>, mut query: Query<&mut Cell>, params: Res<Config>) {
    let between_width = Uniform::from(0..board.width);
    let between_height = Uniform::from(0..board.height);
    let mut rng = rand::thread_rng();
    //println!("{}", query.iter().len());
    for _ in 0..params.dead_ants {
        loop {
            let new_x = between_width.sample(&mut rng);
            let new_y = between_height.sample(&mut rng);
            let mut cell = query.get_mut(board.content[new_x][new_y]).unwrap();
            if !cell.has_dead_ant {
                cell.has_dead_ant = true;
                break;
            }
        }
    }
}

pub fn color_cells(mut query_cell: Query<(&Cell, &mut Sprite), Changed<Cell>>) {
    for (cell, mut sprite) in query_cell.iter_mut() {
        let empty_cell_color: Color = Color::rgb(0.9, 0.45, 0.0);
        let food_cell_color: Color = Color::rgb(0.8, 0.05, 0.0);
        
        sprite.color = if cell.has_dead_ant {food_cell_color} else {empty_cell_color};
    }
}
