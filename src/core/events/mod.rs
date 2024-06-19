pub mod keyboard;
pub mod window;

pub use window::*;
pub use keyboard::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum EventType {
    None,
    WindowClose, WindowResize, WindowFocus, WindowLostFocus, WindowMoved,
    AppTick, AppUpdate, AppRender,
    KeyPressed, KeyReleased,
    MouseButtonPressed, MouseButtonReleased, MouseMoved, MouseScrolled
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum EventCategory {
    None,
    EventCategoryApplication,
    EventCategoryInput,
    EventCategoryKeyboard,
    EventCategoryMouse,
    EventCategoryMouseButton
}

pub trait Event: Copy {
    fn get_name(&self) -> String;
    fn get_event_type(&self) -> EventType;
    fn get_event_category(&self) -> EventCategory;

    fn handled(&self) -> bool;
    fn set_handled(&mut self, handled: bool);

    fn to_string(&self) -> String {
        format!("{} ({:?})", self.get_name(), self.get_event_type())
    }

    fn is_in_category(&self,category: EventCategory) -> bool {
        self.get_event_category() == category
    }
}


pub struct EventDispatcher<T> {
    target: T,
    event_type: fn(T) -> bool,
}

impl<T: Event> EventDispatcher<T> {
    pub fn new(target: T) -> Self {
        Self { target, event_type: |_| false }
    }

    pub fn dispatch(&mut self, mut event: T) {
        if (self.event_type)(self.target) {
            event.set_handled(true);
        }
    }
}
