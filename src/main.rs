use crate::game_loop::run;

mod game_loop;
mod state;
mod buffers;

#[tokio::main]
async fn main() {
    run("My awesome game!").await;
}
