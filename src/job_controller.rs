use crate::proto;
use crate::job_repository;

use crate::proto::distrrr_api::job_service_server::{JobService, JobServiceServer};
use crate::proto::distrrr_api::{CancelJobRequest, CommitTaskRequest, CreateJobRequest, CreateJobResponse, CreateJobTaskRequest, GetJobRequest, GetJobResponse, GetTaskRequest, GetTaskResponse};
use tokio_postgres::{NoTls, Error, Client};
use uuid::Uuid;
use tonic::{transport::Server, Request, Response, Status, Code};

use crate::job_repository::{JobRepository, JobRepositoryImpl};

pub struct JobServiceImpl {
   pub job_repository: JobRepositoryImpl,
}

#[async_trait::async_trait]
impl JobService for JobServiceImpl {
    async fn create_job(&self, request: Request<CreateJobRequest>) -> Result<Response<CreateJobResponse>, Status> {
        let create_job_request = request.into_inner();

        let job_id = Uuid::new_v4();

        let result = self.job_repository.add_job(
            job_id,
            create_job_request.name,
            create_job_request.data,
            create_job_request.retry_count,
        ).await;

        if result.is_some() {
            return Err(Status::new(Code::Internal, result.unwrap().to_string()));
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
