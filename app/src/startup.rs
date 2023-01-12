use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, get, App, HttpServer, Responder};
use sea_orm::{DatabaseConnection, ConnectOptions, Database};
use secrecy::Secret;
use std::net::TcpListener;


use crate::configuration::{DatabaseSettings, Settings};
use crate::routes::*;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database).await;

        let address = format!("{}:{}", configuration.application.host, configuration.application.port);
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();

        let server = run(
            listener,
            configuration.application.base_url,
            configuration.application.hmac_secret,
        )
        .await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub async fn get_connection_pool(configuration: &DatabaseSettings) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(configuration.get_connection_string());

    opt.sqlx_logging(true)
        .sqlx_logging_level(tracing::log::LevelFilter::Info)
        .acquire_timeout(std::time::Duration::from_secs(2));

    Database::connect(opt).await.expect("Could not connect to database")
}


async fn run(
    listener: TcpListener,
    _base_url: String,
    _hmac_secret: Secret<String>,
) -> Result<Server, anyhow::Error> {
   
    let server = HttpServer::new(move || {
        App::new()
            .service(health_check::health_check)
            .service(home::home)
    })
    .listen(listener)?
    .run();
    Ok(server)
}