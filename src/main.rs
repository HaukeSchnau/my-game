use crate::game_loop::run;

mod game_loop;
mod state;
mod texture;
mod camera;
mod camera_controller;
mod instance;
mod model;
mod resources;

#[tokio::main]
async fn main() {
    run("My awesome game!").await;
}
