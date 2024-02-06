use crate::windowing::run;

mod windowing;
mod state;

#[tokio::main]
async fn main() {
    run("My awesome game!").await;
}
