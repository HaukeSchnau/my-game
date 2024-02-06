use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit::event::WindowEvent;

use crate::state::State;

pub async fn run(title: &str) {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut state = State::new(&window).await;

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                elwt.exit();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                state.resize(new_size);
            }
            Event::AboutToWait {} => {}
            _ => ()
        }
    }).expect("Failed to run event loop");
}
