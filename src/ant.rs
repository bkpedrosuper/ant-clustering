use crate::{board::Board, config::Config, cell::Cell};
use rand::{distributions::Uniform, prelude::Distribution, Rng};
use bevy::prelude::*;

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
    mut query: Query<&mut Cell>,
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

            let mut cell = query.get_mut(board.content[x][y]).unwrap();


            if !cell.has_dead_ant {
                cell.has_dead_ant = true;
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
    
    for (mut agent, _) in query.iter_mut() {

        loop {
            let x_move: i32 = rand::thread_rng().gen_range(-1..1);
            let y_move: i32 = rand::thread_rng().gen_range(-1..1);

            
            if !(x_move == 0 && y_move == 0) {
                println!("AQUI?");
                let new_x = agent.x + x_move as usize;
                let new_y = agent.y + y_move as usize;


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

pub fn set_visibility(mut query: Query<(&mut Visibility)>) {
    for mut visibility in query.iter_mut() {
        visibility.is_visible = true;
    }
}