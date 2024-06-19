use super::{Event, EventCategory, EventType};

// EventType::WindowClose

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct WindowResizeEvent {
    pub width: u32,
    pub height: u32,

    _handled: bool,
}

impl WindowResizeEvent {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height, _handled: false }
    }
}

impl Event for WindowResizeEvent {
    fn get_name(&self) -> String {
        "WindowResizeEvent".to_string()
    }

    fn get_event_type(&self) -> EventType {
        EventType::WindowResize
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::EventCategoryApplication
    }

    fn handled(&self) -> bool {
        self._handled
    }

    fn set_handled(&mut self, handled: bool) {
        self._handled = handled;
    }
}
