pub mod app;
pub mod config;
pub mod constants;
pub mod log;

enum ExitErrorCode {
    Config = 1,
    App = 2,
}

impl ExitErrorCode {
    fn exit(self) {
        std::process::exit(self as i32);
    }
}

#[tokio::main]
async fn main() {
    if let Err(e) = config::load() {
        errorln!("failed to initialize the config, error: {}", e.to_string());
        ExitErrorCode::Config.exit();
    }

    log::initialize();

    outputln!("starting");

    if let Err(e) = app::run().await {
        errorln!("error occurred at the api server, error: {}", e.to_string());
        ExitErrorCode::App.exit();
    }
}
