use super::entity::CandlesStruct;
use mockall::{automock, predicate::*};
use sqlx::{Pool, Postgres};

#[cfg_attr(test, automock)]
pub trait CandlesRepositoryTrait {
    async fn get_all_candles(&self) -> Result<Vec<CandlesStruct>, sqlx::Error>;
}

#[derive(Clone)]
pub struct CandlesRepository {
    candles_repo: Pool<Postgres>,
}

impl CandlesRepository {
    pub fn new_candles_repository(candles_repo: Pool<Postgres>) -> Self {
        Self { candles_repo }
    }
}

impl CandlesRepositoryTrait for CandlesRepository {
    async fn get_all_candles(&self) -> Result<Vec<CandlesStruct>, sqlx::Error> {
        let query = r#"
            SELECT id, title, price, color_id FROM public.candles
        "#;

        let candles = sqlx::query_as(query)
            .fetch_all(&self.candles_repo)
            .await
            .map_err(|err| {
                eprintln!("Error fetching candles: {:?}", err);
                err
            })?;

        Ok(candles)
    }
}
