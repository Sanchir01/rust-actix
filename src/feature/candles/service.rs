use std::sync::Arc;

use super::{
    entity::CandlesStruct,
    repository::{CandlesRepository, CandlesRepositoryTrait},
};
pub trait CandlesServiceTrait {
    async fn get_all_candles(&self) -> Result<Vec<CandlesStruct>, sqlx::Error>;
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
}
