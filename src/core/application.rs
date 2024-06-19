use std::ops::Index;

use crate::ev_assert;

use super::window::{self, Window, WindowProps};

pub struct Application {
    running: bool,
    minimized: bool,
    spec: ApplicationSpec,
    last_frame: f64,

    window: Box<dyn Window>,
}

impl Application {
    pub fn new(spec: ApplicationSpec) -> Self {
        let window = window::create_window(WindowProps::default());

        Self {
            running: true,
            minimized: false,
            spec,
            last_frame: 0.0,

            window,
        }
    }

    pub fn push_layer(&mut self) {}

    pub fn on_event() {}

    fn on_window_close(&mut self) {
        self.running = false;
    }

    pub fn run(&mut self) {
        while self.running {
            self.window.on_update();
        }
    }
}

pub struct CommandLineArgs {
    pub count: usize,
    pub args: Vec<String>,
}

impl Index<usize> for CommandLineArgs {
    type Output = String;
    fn index(&self, index: usize) -> &Self::Output {
        ev_assert!(index < self.count);
        &self.args[index]
    }
}

pub struct ApplicationSpec {
    pub name: String,
    pub working_directory: String,
    pub args: CommandLineArgs,
}

impl Default for ApplicationSpec {
    fn default() -> Self {
        Self {
            name: "Eveon Game".to_string(),
            working_directory: "".to_string(),
            args: CommandLineArgs {
                count: 0,
                args: vec![],
            },
        }
    }
}
