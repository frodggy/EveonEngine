use std::sync::mpsc::Receiver;

use glfw::{WindowEvent, WindowMode, Context, Key, Action};

pub struct EvWindow {
    glfw: glfw::Glfw,
    window: glfw::Window,
    vsync: bool,
    events: Receiver<(f64, WindowEvent)>,
}

impl EvWindow {
    pub fn new(title: &str, height: u32, width: u32) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("failed to initialize glfw");

        let (mut window, events) = glfw
            .create_window(width, height, title, WindowMode::Windowed)
            .expect("Failed to create window");

        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);

        Self {
            glfw,
            window,
            vsync: false,
            events,
        }
    }

    pub fn init_gl(&mut self) {
        self.window.make_current();
        gl::load_with(|s| self.window.get_proc_address(s) as *const _)
    }

    pub fn update(&mut self) {
        self.handle_events();
        self.glfw.poll_events();
        self.window.swap_buffers();
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn set_vsync(&mut self, vsync: bool) {
        if vsync {
            self.glfw.set_swap_interval(glfw::SwapInterval::Sync(1))
        } else {
            self.glfw.set_swap_interval(glfw::SwapInterval::Sync(0))
        }
    }

    pub fn is_vsync(&self) -> bool {
        self.vsync
    }

    fn handle_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events)  {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                },
                glfw::WindowEvent::FramebufferSize(height, width) => {
                    unsafe { gl::Viewport(0, 0, width, height) }
                },
                _ => {}
            }
        }
    } 
}
