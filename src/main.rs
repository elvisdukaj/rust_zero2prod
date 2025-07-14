use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let url = format!(
        "{}:{}",
        configuration.server.host, configuration.server.port
    );

    let listener = TcpListener::bind(url);
    run(listener?)?.await
}
