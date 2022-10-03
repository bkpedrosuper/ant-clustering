use crate::{data_board::Board, config::Config, data_cell::DataCell, food::Food};
use rand::{distributions::Uniform, prelude::Distribution, Rng};
use bevy::prelude::*;
use rand::seq::SliceRandom;
// use std::f32::consts::E;

#[derive(Debug, Clone, Component, Default)]
pub struct Ant {
    pub x: usize,
    pub y: usize,
    pub radius: i32,
    pub carrying: Option<Food>,
}

impl Ant {
    pub fn new(pos_x: usize, pos_y: usize, radius: i32) -> Self {
        Self {
            x: pos_x,
            y: pos_y,
            radius,
            carrying: None
        }
    }
}

pub fn setup_ants(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    board: Res<Board>,
    windows: Res<Windows>,
    config: Res<Config>,
) {
    asset_server.watch_for_changes().unwrap();

    let window = windows.primary();
    let cell_width =
        (window.width() - config.border_size * (board.width - 1) as f32) / (board.width as f32);
    let cell_height =
        (window.height() - config.border_size * (board.height - 1) as f32) / (board.height as f32);

    let between_width = Uniform::from(0..board.width);
    let between_height = Uniform::from(0..board.height);
    let mut rng = rand::thread_rng();

    for _ in 0..config.ants {

        loop {
            let x = between_width.sample(&mut rng);
            let y = between_height.sample(&mut rng);

            let xx = x as f32;
            let yy = y as f32;
            let cx = -window.width() / 2. + cell_width * xx + config.border_size * xx + cell_width / 2.;
            let cy =
                -window.height() / 2. + cell_height * yy + config.border_size * yy + cell_height / 2.;
            commands
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load("empty_ant.png"),
                    transform: Transform::from_xyz(cx, cy, 2.0),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(cell_width, cell_height)),
                        ..default()
                    },
                    ..default()
                })
                .insert(Ant {
                    x,
                    y,
                    radius: config.radius,
                    carrying: None,
                    ..default()
                });

            break;
        }
    }
}

fn get_score(
    board: &Res<Board>,
    ax: i32,
    ay: i32,
    radius: i32,
    item: Food,
    query_cell: &Query<&mut DataCell>,
) -> f32 {
    let width = board.width as i32;
    let height = board.height as i32;
    let mut total_vision = 0;
    let mut similarity_sum: f32 = 0.0;
    for x in ax - radius..=ax + radius {
        for y in ay - radius..=ay + radius {
            if x >= 0 && x < width && (x != ax || y != ay) && y >= 0 && y < height
            {
                total_vision += 1;
                let cell = query_cell.get(board.content[x as usize][y as usize]).unwrap();
                match cell.food {
                    Some(food) => {
                        similarity_sum += 1. / Food::dist(item, food)
                    },
                    None => {}
                }
            }
        }
    }

    similarity_sum as f32 / total_vision as f32
}

fn pick_function(agent: &mut Mut<Ant>, score: f32, board: &Res<Board>, query_cell: &mut Query<&mut DataCell>, config: &Config) {
    let pick_prob = (config.k1 / (config.k1 + score)).powi(2);

    let random_value: f32 = rand::thread_rng().gen();

    let mut cell = query_cell.get_mut(board.content[agent.x][agent.y]).unwrap();

    if random_value <= pick_prob {
        // pick food
        agent.carrying = cell.food;
        cell.food = None;
    }
}

fn drop_function(agent: &mut Mut<Ant>, score: f32, board: &Res<Board>, query_cell: &mut Query<&mut DataCell>, config: &Config) {
    let drop_prob = (score / (config.k2 + score)).powi(2);

    let random_value: f32 = rand::thread_rng().gen();

    let mut cell = query_cell.get_mut(board.content[agent.x][agent.y]).unwrap();

    if random_value <= drop_prob {
        cell.food = agent.carrying;
        agent.carrying = None;
    }
}

pub fn move_agent(
    windows: Res<Windows>,
    board: Res<Board>,
    mut query: Query<(&mut Ant, &mut Transform)>,
    mut query_cell: Query<&mut DataCell>,
    mut config: ResMut<Config>,
) {
    let window = windows.primary();

    let cell_width =
        (window.width() - config.border_size * (board.width - 1) as f32) / (board.width as f32);
    let cell_height =
        (window.height() - config.border_size * (board.height - 1) as f32) / (board.height as f32);

    for _ in 0..config.iter_per_mut {

        for (mut agent, _) in query.iter_mut() {

            if config.finished && match agent.carrying {Some(_) => false, None => true} {
                continue;
            }

            let current_cell = query_cell.get_mut(board.content[agent.x][agent.y]).unwrap();

            match agent.carrying {
                // Drop the food
                Some(food) => {
                    match current_cell.food {
                        Some(_) => {}
                        None => {
                            let score: f32 = get_score(&board, agent.x as i32, agent.y as i32, config.radius, food , &query_cell).try_into().unwrap();

                            drop_function(&mut agent, score, &board, &mut query_cell, &config)
                        }
                    }
                }
                // Pick the food
                None => {
                    match current_cell.food {
                        Some(cell_food) => {
                            let score: f32 = get_score(&board, agent.x as i32, agent.y as i32, config.radius, cell_food , &query_cell).try_into().unwrap();
                            
                            pick_function(&mut agent, score, &board, &mut query_cell, &config)
                        }
                        None => {}
                    }
                }
            }
            
            // agent_action(&mut agent, score, &board, &mut query_cell);
    
            loop {
                let mut moves_available: Vec<(usize, usize)> = Vec::new();
    
                // pode ir pra direita
                if agent.x < board.width -1 {
                    moves_available.push((agent.x + 1, agent.y));
                }
    
                // pode ir pra esquerda
                if agent.x > 0 {
                    moves_available.push((agent.x - 1, agent.y));
                }
    
                // pode ir pra cima
                if agent.y < board.height -1 {
                    moves_available.push((agent.x, agent.y + 1));
                }
    
                // pode ir pra baixo
                if agent.y > 0 {
                    moves_available.push((agent.x, agent.y - 1));
                }
    
                
                let movement= moves_available.choose(&mut rand::thread_rng()).expect("error while attempting to move");
    
                let new_x = movement.0;
                let new_y = movement.1;
    
                if new_x < board.width && new_y < board.height {
                    // let mut cell = query_cell.get_mut(board.content[new_x][new_y]).unwrap();
                    agent.x = new_x;
                    agent.y = new_y;
                    break;
                }
                
            }
        }

        config.iterations = if config.iterations == 0 { 0 } else { config.iterations - 1 }; // command to prevent config (usize to be lesser than zero)
    }

    if config.iterations == 0 {
        config.finished = true;
    }

    
    for (agent, mut transform) in query.iter_mut() {
        let x = agent.x as f32;
        let y = agent.y as f32;
        let cx = -window.width() / 2. + cell_width * x + config.border_size * x + cell_width / 2.;
        let cy = -window.height() / 2. + cell_height * y + config.border_size * y + cell_height / 2.;
        let translation = &mut transform.translation;
        translation.x = cx;
        translation.y = cy;
    }

}

pub fn draw_agents(asset_server: Res<AssetServer>, mut query: Query<(&Ant, &mut Handle<Image>)>) {
    for (agent, mut image_handle) in query.iter_mut() {
        match agent.carrying {
            Some (_)=> {*image_handle = asset_server.load("carry_ant.png")},
            None => *image_handle = asset_server.load("empty_ant.png")
        }
    }
}