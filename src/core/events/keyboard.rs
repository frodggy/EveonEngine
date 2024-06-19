use super::{Event, EventCategory, EventType};

pub trait KeyEvent {
    fn get_key_code(&self) -> i32;
}

// EventType::KeyPressed

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct KeyPressedEvent {
    pub key_code: i32,
    _handled: bool,
}

impl KeyPressedEvent {
    pub fn new(key_code: i32) -> Self {
        Self { key_code, _handled: false }
    }
}

impl Event for KeyPressedEvent {
    fn get_name(&self) -> String {
        "KeyPressedEvent".to_string()
    }

    fn get_event_type(&self) -> EventType {
        EventType::KeyPressed
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::EventCategoryInput
    }

    fn handled(&self) -> bool {
        self._handled
    }

    fn set_handled(&mut self, handled: bool) {
        self._handled = handled;
    }
}

impl KeyEvent for KeyPressedEvent {
    fn get_key_code(&self) -> i32 {
        self.key_code
    }
}

// EventType::KeyReleased

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct KeyReleasedEvent {
    pub key_code: i32,
    _handled: bool,
}

impl KeyReleasedEvent {
    pub fn new(key_code: i32) -> Self {
        Self { key_code, _handled: false }
    }
}

impl Event for KeyReleasedEvent {
    fn get_name(&self) -> String {
        "KeyReleasedEvent".to_string()
    }

    fn get_event_type(&self) -> EventType {
        EventType::KeyReleased
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::EventCategoryInput
    }

    fn handled(&self) -> bool {
        self._handled
    }

    fn set_handled(&mut self, handled: bool) {
        self._handled = handled;
    }
}

impl KeyEvent for KeyReleasedEvent {
    fn get_key_code(&self) -> i32 {
        self.key_code
    }
}
