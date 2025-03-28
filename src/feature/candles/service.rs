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
        slug: &str,
        color_id: Uuid,
    ) -> Result<Uuid, Box<dyn std::error::Error>>;
}

#[derive(Clone)]
pub struct CandlesService {
    candles_service: Arc<CandlesRepository>,
}

impl CandlesService {
    pub fn new_candles_services(candles_service: Arc<CandlesRepository>) -> Self {
        Self { candles_service }
    }
}

impl CandlesServiceTrait for CandlesService {
    async fn get_all_candles(&self) -> Result<Vec<CandlesStruct>, sqlx::Error> {
        self.candles_service.get_all_candles().await
    }
    async fn create_candle(
        &self,
        title: &str,
        slug: &str,
        color_id: Uuid,
    ) -> Result<Uuid, Box<dyn std::error::Error>> {
        let user_id = self
            .candles_service
            .create_candle(title, slug, color_id)
            .await?;

        Ok(user_id)
    }
}
