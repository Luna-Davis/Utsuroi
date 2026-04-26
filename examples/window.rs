// This helps the analyzer realize it's linked to your library
use utsuroi::*;
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.run_app(&mut App::new()).expect("Failed to run app");
}
