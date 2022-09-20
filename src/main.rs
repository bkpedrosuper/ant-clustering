use ant_clustering::ant::Ant;
use ant_clustering::board::{*};
use ant_clustering::config::Config;
use bevy::prelude::*;
use bevy::window::PresentMode;
use rand::Rng;


fn main() {
    let params: Config = Config { dead_ants: 100, max_iter: 10, ants: 15, radius: 1 };
    let mut board: Board = Board::new(20, 20);
    board = board.populate_board(&params);

    // Create ants inside the board
    let ants: Vec<Ant> = vec![Ant::new(
        rand::thread_rng().gen_range(0..board.height),
        rand::thread_rng().gen_range(0..board.width),
        1); params.ants];

    for _ in 0..params.max_iter {
        // for ant in &ants {
        //     ant.moveit(&mut board)
        // }
    }

    board.print_board();

    println!("Hello, world!");
}
