pub use spdlog::{debug, error, info, trace, warn};
use std::sync::Arc;

pub static mut CLIENT_LOGGER: Option<Arc<spdlog::Logger>> = None;
pub static mut ENGINE_LOGGER: Option<Arc<spdlog::Logger>> = None;

pub struct Log;

pub(crate) mod engine {
    #[macro_export]
    macro_rules! engine_info {
        ($($arg:tt)*) => {
            use crate::core::log::ENGINE_LOGGER;
            use crate::core::log::info;
            unsafe {
                match &ENGINE_LOGGER {
                    Some(logger) => {
                        info!(logger: logger, $($arg)*);
                    },
                    None => {
                        eprintln!("Must initialize logger first!");
                    }
                }
            }
        }
    }

    #[macro_export]
    macro_rules! engine_warn {
        ($($arg:tt)*) => {
         unsafe {
                use crate::core::log::ENGINE_LOGGER;
                use crate::core::log::warn;
                match &ENGINE_LOGGER {
                    Some(logger) => {
                       warn!(logger: logger, $($arg)*);
                    },
                    None => {
                        eprintln!("Must initialize logger first!");
                    }
                }
            }
        }
    }

    #[macro_export]
    macro_rules! engine_error {
        ($($arg:tt)*) => {
            unsafe {
                use crate::core::log::ENGINE_LOGGER;
                use crate::core::log::error;
                match &ENGINE_LOGGER {
                    Some(logger) => {
                        error!(logger: logger, $($arg)*);
                    },
                    None => {
                        eprintln!("Must initialize logger first!");
                    }
                }
            }
        }
    }

    #[macro_export]
    macro_rules! engine_debug {
        ($($arg:tt)*) => {
            unsafe {
                use crate::core::log::ENGINE_LOGGER;
                use crate::core::log::debug;
                match &ENGINE_LOGGER {
                    Some(logger) => {
                        debug!(logger: logger, $($arg)*);
                    },
                    None => {
                        eprintln!("Must initialize logger first!");
                    }
                }
            }
        }
    }

    #[macro_export]
    macro_rules! engine_trace {
        ($($arg:tt)*) => {
            unsafe {
                use crate::core::log::ENGINE_LOGGER;
                use crate::core::log::trace;
                match &ENGINE_LOGGER {
                    Some(logger) => {
                        trace!(logger: logger, $($arg)*);
                    },
                    None => {
                        eprintln!("Must initialize logger first!");
                    }
                }
            }
        }
    }
}

#[macro_use]
mod client {
    #[macro_export]
    macro_rules! ev_info {
        ($($arg:tt)*) => {
            unsafe {
                use eveon::core::log::CLIENT_LOGGER;
                use eveon::core::log::info;
                match &CLIENT_LOGGER {
                    Some(logger) => {
                        info!(logger: logger, $($arg)*);
                    },
                    None => {
                        eprintln!("Must initialize logger first!");
                    }
                }
            }
        }
    }

    #[macro_export]
    macro_rules! ev_warn {
        ($($arg:tt)*) => {
            use eveon::core::log::CLIENT_LOGGER;
            use eveon::core::log::warn;
            unsafe {
                match &CLIENT_LOGGER {
                    Some(logger) => {
                        warn!(logger: logger, $($arg)*);
                    },
                    None => {
                        eprintln!("Must initialize logger first!");
                    }
                }
            }
        }
    }

    #[macro_export]
    macro_rules! ev_error {
        ($($arg:tt)*) => {
            unsafe {
                use eveon::core::log::CLIENT_LOGGER;
                use eveon::core::log::error;
                match &CLIENT_LOGGER {
                    Some(logger) => {
                        error!(logger: logger, $($arg)*);
                    },
                    None => {
                        eprintln!("Must initialize logger first!");
                    }
                }
            }
        }
    }

    #[macro_export]
    macro_rules! ev_debug {
        ($($arg:tt)*) => {
            use eveon::core::log::CLIENT_LOGGER;
            use eveon::core::log::debug;
            unsafe {
                match &CLIENT_LOGGER {
                    Some(logger) => {
                        debug!(logger: logger, $($arg)*);
                    },
                    None => {
                        eprintln!("Must initialize logger first!");
                    }
                }
            }
        }
    }

    #[macro_export]
    macro_rules! ev_trace {
        ($($arg:tt)*) => {
            unsafe {
                use eveon::core::log::CLIENT_LOGGER;
                use eveon::core::log::trace;
                match &CLIENT_LOGGER {
                    Some(logger) => {
                        trace!(logger: logger, $($arg)*);
                    },
                    None => {
                        eprintln!("Must initialize logger first!");
                    }
                }
            }
        }
    }

    pub use ev_debug;
    pub use ev_error;
    pub use ev_info;
    pub use ev_trace;
    pub use ev_warn;
}

pub use client::*;

impl Log {
    pub fn init() {
        let sinks = spdlog::default_logger().sinks().to_owned();
        let mut builder = spdlog::Logger::builder();
        let builder: &mut spdlog::LoggerBuilder =
            builder.sinks(sinks).level_filter(spdlog::LevelFilter::All);

        let client_logger = builder.name("client").build().unwrap();
        let engine_logger = builder.name("engine").build().unwrap();

        unsafe {
            CLIENT_LOGGER = Some(Arc::new(client_logger));
            ENGINE_LOGGER = Some(Arc::new(engine_logger));
        }
    }
}
