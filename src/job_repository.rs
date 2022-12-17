use std::io::Empty;
use tokio_postgres::{NoTls, Error, Client};
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
        let retry_count = if retry_count.is_some() {
            retry_count.unwrap()
        } else {
            0
        };
        let result = self.client
            .query("INSERT INTO distrrr.job(id, name, data, retry_count) VALUES ($1, $2, $3, $4)",
                   &[&job_id, &name, &data, &retry_count],
            ).await;

        return result.err();
    }
}
