
pub fn create_window(spec: WindowProps) -> Box<dyn Window> {
    #[cfg(feature = "opengl")]
    let window = crate::backends::opengl::create_window(spec);
    #[cfg(feature = "vulkan")]
    let window = crate::backends::vulkan::VkWindow::create_window(spec);

    Box::new(window)
}

pub trait Window {
    fn create(&mut self);

    fn get_height(&self) -> u32;
    fn get_width(&self) -> u32;

    fn on_update(&mut self);

    fn vsync(&self) -> bool;
    fn set_vsync(&mut self, vsync: bool);

    // fn set_event_callback(&mut self, callback: fn(EventType));
}

pub struct WindowProps {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Default for WindowProps {
    fn default() -> Self {
        Self {
            title: "Eveon Game".to_string(),
            width: 800,
            height: 600,
        }
    }
}
