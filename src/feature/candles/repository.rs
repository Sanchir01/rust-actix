use super::entity::CandlesStruct;
use mockall::{automock, predicate::*};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[cfg_attr(test, automock)]
pub trait CandlesRepositoryTrait {
    async fn get_all_candles(&self) -> Result<Vec<CandlesStruct>, sqlx::Error>;
    async fn create_candle(
        &self,
        title: &str,
        slug: &str,
        color_id: Uuid,
    ) -> Result<Uuid, sqlx::Error>;
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
    async fn create_candle(
        &self,
        title: &str,
        slug: &str,
        color_id: Uuid,
    ) -> Result<Uuid, sqlx::Error> {
        let query = r#"
            INSERT INTO candles (title, slug, color_id) VALUES ($1,$2, $3) RETURNING id
        "#;
        let candle_id: Uuid = sqlx::query_scalar(query)
            .bind(title)
            .bind(slug)
            .bind(color_id)
            .fetch_one(&self.candles_repo)
            .await
            .map_err(|e| {
                eprintln!("error creating: {:?}", e);
                e
            })?;

        Ok(candle_id)
    }
}
