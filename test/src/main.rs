// use std::path::Path;

// use eveon_engine::project::Project;
use eveon_engine::core::graphics::window::*;

fn main() {
    let mut window = EvWindow::new("My Game", 1080, 720);

    window.init_gl();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.3, 0.5, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.update();
    }
}
