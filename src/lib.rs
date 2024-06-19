mod backends;
pub mod core;

#[macro_export]
macro_rules! create_application {
    () => {
        fn main() {
            use eveon::core::application::*;
            use std::env;

            let args = env::args().collect::<Vec<String>>();
            let command_line_args = CommandLineArgs {
                count: args.len(),
                args,
            };

            eveon::core::log::Log::init();

            let mut application: Application = ev_main(command_line_args);

            application.run();
        }
    };
}

#[macro_export]
macro_rules! ev_assert {
    ($($arg:tt)*) => {
        if !($($arg)*) {
            crate::engine_error!("Assertion failed: {}", stringify!($($arg)*));
            std::process::exit(1);
        }
    };
}

#[macro_export]
macro_rules! ev_panic {
    ($($arg:tt)*) => {
        crate::engine_error!($($arg)*);
        std::process::exit(1);
    };
}
