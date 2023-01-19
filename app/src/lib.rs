mod configuration;
mod telemetry;
mod startup;
mod routes;
mod db;
mod utils;
mod auth;


#[tokio::main]
pub async fn application_start_and_run() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("testserver".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let application = startup::Application::build(configuration.clone()).await.expect("Application startup failed");

    application.run_until_stopped().await?;
    Ok(())
}
