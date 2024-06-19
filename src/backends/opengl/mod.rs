use crate::{
    core::{events::{Event, WindowResizeEvent}, window::Window},
    engine_info, ev_panic,
};
use glfw::Context;

pub struct WindowData {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub vsync: bool,

    pub event_callback: fn(),
}

pub struct GlWindow {
    data: WindowData,

    glfw_window: glfw::PWindow,
}

pub fn create_window(props: crate::core::window::WindowProps) -> GlWindow {
    let mut glfw = match glfw::init(glfw::fail_on_errors) {
        Ok(glfw) => glfw,
        Err(err) => {
            ev_panic!("Failed to initialize GLFW: {}", err);
        }
    };

    engine_info!(
        "Creating window {} {}x{}",
        props.title,
        props.width,
        props.height
    );

    let data = WindowData {
        title: props.title,
        width: props.width,
        height: props.height,
        vsync: true,
        event_callback: || {},
    };

    let (mut glfw_window, _events) = glfw
        .create_window(
            data.width,
            data.height,
            &data.title,
            glfw::WindowMode::Windowed,
        )
        .unwrap();

    glfw_window.make_current();
    glfw_window.set_all_polling(true);
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    GlWindow { data, glfw_window }
}

impl Window for GlWindow {
    fn get_height(&self) -> u32 {
        self.data.height
    }

    fn get_width(&self) -> i32 {
        self.data.width
    }

    fn on_update(&mut self) {
        self.glfw_window.swap_buffers();
    }

    fn vsync(&self) -> bool {
        self.data.vsync
    }

    fn set_vsync(&mut self, vsync: bool) {
        self.data.vsync = vsync;
    }

    fn create(&mut self) {
        self.glfw_window.set_size_callback(|_window, width, height| {
            self.data.width = width;
            self.data.height = height;
            self.data.event_callback();
            WindowResizeEvent::new(width, height)
        })
    }

    // fn set_event_callback(&mut self, callback: fn(crate::core::events::EventType)) {
    //     todo!()
    // }
}
