use std::sync::Arc;

use uuid::Uuid;

use super::{
    entity::CandlesStruct,
    repository::{CandlesRepository, CandlesRepositoryTrait},
};
pub trait CandlesServiceTrait {
    async fn get_all_candles(&self) -> Result<Vec<CandlesStruct>, sqlx::Error>;
    async fn create_candle(
        &self,
        title: &str,
        price: i64,
        version: i64,
        color_id: Uuid,
    ) -> Result<Uuid, Box<dyn std::error::Error>>;
}

#[derive(Clone)]
pub struct CandlesService {
    candles_repo: Arc<CandlesRepository>,
}

impl CandlesService {
    pub fn new_candles_services(candles_repo: Arc<CandlesRepository>) -> Self {
        Self { candles_repo }
    }
}

impl CandlesServiceTrait for CandlesService {
    async fn get_all_candles(&self) -> Result<Vec<CandlesStruct>, sqlx::Error> {
        self.candles_repo.get_all_candles().await
    }
    async fn create_candle(
        &self,
        title: &str,
        price: i64,
        version: i64,
        color_id: Uuid,
    ) -> Result<Uuid, Box<dyn std::error::Error>> {
        let user_id = self
            .candles_repo
            .create_candle(title, price, version, color_id)
            .await?;

        Ok(user_id)
    }
}
