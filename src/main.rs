use ant_clustering::board::{*};
use ant_clustering::config::Config;
// use bevy::diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;
use ant_clustering::ant::{*};


fn main() {
    let params: Config = Config { dead_ants: 1000, max_iter: 10, ants: 100, radius: 1, border_size: 2., iter_per_mut: 100};
    let board: Board = Board::new(150);

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
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
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
        .add_system(draw_ants)
        // .add_system(set_visibility)
        .run();

}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}