use tonic::{transport::Server, Request, Response, Status};
use distrrr_api::job_service_server::{JobService, JobServiceServer};
use crate::distrrr_api::{CommitTaskRequest, CreateJobResponse, CreateJobTaskRequest, GetJobRequest, GetJobResponse, GetTaskRequest, GetTaskResponse};
use uuid::Uuid;

pub mod distrrr_api {
    tonic::include_proto!("dev.mazuninky.distrrr");
}

#[derive(Debug, Default)]
pub struct JobServiceImpl {}

#[tonic::async_trait]
impl JobService for JobServiceImpl {
    async fn create_job(
        &self,
        request: Request<()>,
    ) -> Result<Response<CreateJobResponse>, Status> {
        println!("Got a request: {:?}", request);

        let response = CreateJobResponse {
            job_id: Uuid::new_v4().to_string()
        };

        Ok(Response::new(response))
    }

    async fn create_job_task(&self, request: Request<CreateJobTaskRequest>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn get_job(&self, request: Request<GetJobRequest>) -> Result<Response<GetJobResponse>, Status> {
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
    let addr = "[::1]:8080".parse()?;
    let job_service = JobServiceImpl::default();

    Server::builder()
        .add_service(JobServiceServer::new(job_service))
        .serve(addr)
        .await?;

    Ok(())
}
