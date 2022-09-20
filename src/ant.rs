use crate::{board::Board, config::Config, cell::Cell};
use rand::{Rng, distributions::Uniform, prelude::Distribution};
use bevy::prelude::*;

#[derive(Debug, Clone, Component)]
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

pub fn setup_agents(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    board: Res<Board>,
    windows: Res<Windows>,
    mut query: Query<&mut Cell>,
    params: Res<Config>,
) {
    asset_server.watch_for_changes().unwrap();

    let window = windows.primary();
    let border_width = 2.0;
    let cell_width =
        (window.width() - border_width * (board.width - 1) as f32) / (board.width as f32);
    let cell_height =
        (window.height() - border_width * (board.height - 1) as f32) / (board.height as f32);

    let mut cont = 0;
    let between_width = Uniform::from(0..board.width);
    let between_height = Uniform::from(0..board.height);
    let mut rng = rand::thread_rng();

    while cont < params.ants {
        let x = between_width.sample(&mut rng);
        let y = between_height.sample(&mut rng);
        let mut cell = query.get_mut(board.content[x][y]).unwrap();
        if !cell.has_dead_ant {
            cell.has_dead_ant = true;
            let xx = x as f32;
            let yy = y as f32;
            let cx = -window.width() / 2. + cell_width * xx + border_width * xx + cell_width / 2.;
            let cy =
                -window.height() / 2. + cell_height * yy + border_width * yy + cell_height / 2.;
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
                    radius: params.radius,
                    carrying: false,
                    ..default()
                });
            cont += 1;
        }
    }
}
