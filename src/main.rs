use ant_clustering::board::{*};
use ant_clustering::config::Config;
use bevy::prelude::*;
use bevy::window::PresentMode;
use ant_clustering::ant::{*};


fn main() {
    let params: Config = Config { dead_ants: 100, max_iter: 10, ants: 15, radius: 1, border_size: 2.,};
    let board: Board = Board::new(20, 20);

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
        // .add_system(move_agent)
        .add_system(draw_ants)
        .add_system(color_cells)
        .add_system(set_visibility)
        .run()
        ;

    // // Create ants inside the board
    // let ants: Vec<Ant> = vec![Ant::new(
    //     rand::thread_rng().gen_range(0..board.height),
    //     rand::thread_rng().gen_range(0..board.width),
    //     1); params.ants];

    // for _ in 0..params.max_iter {
    //     // for ant in &ants {
    //     //     ant.moveit(&mut board)
    //     // }
    // }

    // board.print_board();

}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}