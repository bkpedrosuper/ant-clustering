use ant_clustering::board::{*};
use ant_clustering::config::Config;
// use bevy::diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;
use ant_clustering::ant::{*};


fn main() {
    let params: Config = Config::new(3000, 100000, 100, 5, 2., 1000);
    // let params: Config = Config::new(1000, 10000, 100, 1, 2., 100);
    // let params: Config = Config::new(200, 10, 50, 3, 2., 10000);
    let board: Board = Board::new(100);

    App::new()
        .insert_resource(WindowDescriptor {
            title: "Ant-Clustering".to_string(),
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