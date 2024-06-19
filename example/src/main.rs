use eveon::core::application::*;
use eveon::core::log::ev_info;


fn ev_main(args: CommandLineArgs) -> Application {

    let spec = ApplicationSpec {
        name: "My Game".to_string(),
        working_directory: ".".to_string(),
        args,
    };

    let my_game = Application::new(spec);

    ev_info!("Hello, world!");

    my_game
}

eveon::create_application!();
