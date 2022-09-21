use crate::{board::Board, config::Config, cell::Cell};
use rand::{distributions::Uniform, prelude::Distribution, Rng};
use bevy::prelude::*;
use rand::seq::SliceRandom;

#[derive(Debug, Clone, Component, Default)]
pub struct Ant {
    pub x: usize,
    pub y: usize,
    pub radius: i32,
    pub carrying: bool,
}

impl Ant {
    pub fn new(pos_x: usize, pos_y: usize, radius: i32) -> Self {
        Self {
            x: pos_x,
            y: pos_y,
            radius,
            carrying: false
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
                    carrying: false,
                    ..default()
                });

            break;
        }
    }
}

pub fn draw_ants(asset_server: Res<AssetServer>, mut query: Query<(&Ant, &mut Handle<Image>)>) {
    for (ant, mut image_handle) in query.iter_mut() {
        if ant.carrying {
            *image_handle = asset_server.load("carry_ant.png");
        } else {
            *image_handle = asset_server.load("empty_ant.png");
        }
    }
}

fn get_score(
    board: &Res<Board>,
    ax: i32,
    ay: i32,
    radius: i32,
    query_cell: &Query<&mut Cell>,
) -> f32 {
    let width = board.width as i32;
    let height = board.height as i32;
    let mut total_vision = 0;
    let mut food_nearby = 0;
    for x in ax - radius..=ax + radius {
        for y in ay - radius..=ay + radius {
            if x >= 0 && x < width && (x != ax || y != ay) && y >= 0 && y < height
            {
                total_vision += 1;
                let cell = query_cell.get(board.content[x as usize][y as usize]).unwrap();
                if cell.has_dead_ant {
                    food_nearby += 1;
                }
            }
        }
    }

    food_nearby as f32 / total_vision as f32
}

fn agent_action(agent: &mut Mut<Ant>, score: f32, board: &Res<Board>, query_cell: &mut Query<&mut Cell>,) {
    let factor = 1.5;
    let prob = (score * factor).min(1.0);
    let mut rng = rand::thread_rng();

    let random_value: f32 = rng.gen();

    let mut cell = query_cell.get_mut(board.content[agent.x][agent.y]).unwrap();

    if agent.carrying {
        // drop the food
        if random_value < prob {
            if !cell.has_dead_ant {
                agent.carrying = false;
                cell.has_dead_ant = true;
            }
        }
    }
    else {
        // pick the food
        if random_value < (1.0 - prob) {
            if cell.has_dead_ant {
                agent.carrying = true;
                cell.has_dead_ant = false;
            }
        }
    }

}

pub fn move_agent(
    windows: Res<Windows>,
    board: Res<Board>,
    mut query: Query<(&mut Ant, &mut Transform)>,
    mut query_cell: Query<&mut Cell>,
    config: ResMut<Config>,
) {
    let window = windows.primary();

    let cell_width =
        (window.width() - config.border_size * (board.width - 1) as f32) / (board.width as f32);
    let cell_height =
        (window.height() - config.border_size * (board.height - 1) as f32) / (board.height as f32);

    for _ in 0..config.iter_per_mut {

        for (mut agent, _) in query.iter_mut() {
    
            let score: f32 = get_score(&board, agent.x as i32, agent.y as i32, config.radius, &query_cell).try_into().unwrap();
            
            agent_action(&mut agent, score, &board, &mut query_cell);
    
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
    
                
                let movement= moves_available.choose(&mut rand::thread_rng()).expect("erro");
    
                let new_x = movement.0;
                let new_y = movement.1;
    
                if new_x < board.width && new_y < board.height {
                    let mut cell = query_cell.get_mut(board.content[new_x][new_y]).unwrap();
                    cell.n_ants += 1;
                    agent.x = new_x;
                    agent.y = new_y;
                    break;
                }
                
            }
        }
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
        if agent.carrying {
            *image_handle = asset_server.load("carry_ant.png");
        } else {
            *image_handle = asset_server.load("empty_ant.png");
        }
    }
}

pub fn set_visibility(mut query: Query<(&mut Visibility, &Ant)>) {
    for (mut visibility, ant) in query.iter_mut() {
        visibility.is_visible = ant.carrying;
    }
}