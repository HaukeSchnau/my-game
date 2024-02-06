use crate::game_loop::run;

mod game_loop;
mod state;
mod buffers;
mod texture;
mod camera;
mod camera_controller;
mod instance;

#[tokio::main]
async fn main() {
    run("My awesome game!").await;
}
