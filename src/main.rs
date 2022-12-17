mod proto;
mod job_controller;
mod job_repository;
mod entity;
mod settings;

use tonic::{transport::Server, Request, Response, Status, Code};
use proto::distrrr_api::job_service_server::{JobServiceServer};
use tokio_postgres::{NoTls, Error, Client};
use crate::job_repository::JobRepositoryImpl;
use crate::settings::Settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Settings::new().unwrap();

    let (client, connection) =
        tokio_postgres::connect(&config.postgres.connect_string, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let job_service = job_controller::JobServiceImpl { job_repository: JobRepositoryImpl { client } };

    let addr = format!("[::1]:{}", config.port).parse()?;
    Server::builder()
        .add_service(JobServiceServer::new(job_service))
        .serve(addr)
        .await?;

    Ok(())
}
