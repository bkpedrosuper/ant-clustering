use ant_clustering::config::Config;
use ant_clustering::data_board::*;
// use bevy::diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use ant_clustering::data_ant::*;
use bevy::prelude::*;
use bevy::window::PresentMode;

fn main() {

    let params: Config = Config {
        dead_ants: 1000,
        iterations: 100005,
        ants: 20,
        radius: 3,
        border_size: 2.,
        iter_per_mut: 50,
        finished: false,
        base: "Square1-DataSet-400itens.txt".to_string(),
        k1: 0.3,
        k2: 0.3,
        alpha: 29.,
    };
    // let params: Config = Config::new(200, 10, 50, 3, 2., 10000);
    let board: Board = Board::new(50);

    App::new()
        .insert_resource(WindowDescriptor {
            title: "Ant-Clustering Data".to_string(),
            width: 900.,
            height: 900.,
            resizable: false,
            present_mode: PresentMode::Immediate,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<Board>()
        .insert_resource(board)
        .insert_resource(params)
        .add_startup_system_to_stage(StartupStage::PreStartup, setup_board)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_dead_ants)
        .add_startup_system(setup_ants)
        .add_system(color_cells)
        .add_system(draw_agents)
        .add_system(move_agent)
        .run();
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
