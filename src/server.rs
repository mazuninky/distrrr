use postgres_types::{Format, IsNull, ToSql, Type};
use postgres_types::private::BytesMut;
use tonic::{transport::Server, Request, Response, Status, Code};
use distrrr_api::job_service_server::{JobService, JobServiceServer};
use crate::distrrr_api::{CancelJobRequest, CommitTaskRequest, CreateJobRequest, CreateJobResponse, CreateJobTaskRequest, GetJobRequest, GetJobResponse, GetTaskRequest, GetTaskResponse};
use uuid::Uuid;
use tokio_postgres::{NoTls, Error, Client};

pub mod distrrr_api {
    tonic::include_proto!("dev.mazuninky.distrrr");
}

#[derive(Debug)]
struct JobServiceImpl {
    client: Client,
}

enum JobStatus {
    Waiting,
    Processing,
    Competed,
    Error,
    Canceled,
}

#[tonic::async_trait]
impl JobService for JobServiceImpl {
    async fn create_job(&self, request: Request<CreateJobRequest>) -> Result<Response<CreateJobResponse>, Status> {
        let create_job_request = request.into_inner();

        let job_id = Uuid::new_v4();

        let result = self.client
            .query("INSERT INTO distrrr.job(id, name, data) VALUES ($1, $2, $3)",
                   &[&job_id, &create_job_request.name, &create_job_request.data],
            ).await;

        if result.is_err() {
            return Err(Status::new(Code::Internal, result.err().unwrap().to_string()));
        }

        let response = CreateJobResponse { job_id: job_id.to_string() };

        Ok(Response::new(response))
    }

    async fn create_job_task(&self, request: Request<CreateJobTaskRequest>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn get_job(&self, request: Request<GetJobRequest>) -> Result<Response<GetJobResponse>, Status> {
        todo!()
    }

    async fn cancel_job(&self, request: Request<CancelJobRequest>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn get_task(&self, request: Request<GetTaskRequest>) -> Result<Response<GetTaskResponse>, Status> {
        todo!()
    }

    async fn commit_task(&self, request: Request<CommitTaskRequest>) -> Result<Response<()>, Status> {
        todo!()
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=postgres", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let job_service = JobServiceImpl { client };

    let addr = "[::1]:8080".parse()?;
    Server::builder()
        .add_service(JobServiceServer::new(job_service))
        .serve(addr)
        .await?;

    Ok(())
}
