use std::io::Empty;
use tokio_postgres::{Error, Client};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait JobRepository {
    async fn add_job(
        &self,
        job_id: Uuid,
        name: Option<String>,
        data: Option<Vec<u8>>,
        retry_count: Option<u32>,
    ) -> Option<Error>;
}

pub struct JobRepositoryImpl {
    pub client: Client,
}

#[async_trait::async_trait]
impl JobRepository for JobRepositoryImpl {
    async fn add_job(
        &self,
        job_id: Uuid,
        name: Option<String>,
        data: Option<Vec<u8>>,
        retry_count: Option<u32>,
    ) -> Option<Error> {
        let result = self.client
            .query("INSERT INTO distrrr.job(id, name, data, retry_count) VALUES ($1, $2, $3, $4)",
                   &[&job_id, &name, &data, &retry_count.unwrap_or(0)],
            ).await;

        return result.err();
    }
}
