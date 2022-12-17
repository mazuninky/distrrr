mod proto;
mod job_controller;
mod job_repository;
mod entity;

use tonic::{transport::Server, Request, Response, Status, Code};
use proto::distrrr_api::job_service_server::{JobServiceServer};
use tokio_postgres::{NoTls, Error, Client};
use crate::job_repository::JobRepositoryImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=postgres", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let job_service = job_controller::JobServiceImpl { job_repository: JobRepositoryImpl { client } };

    let addr = "[::1]:8080".parse()?;
    Server::builder()
        .add_service(JobServiceServer::new(job_service))
        .serve(addr)
        .await?;

    Ok(())
}
