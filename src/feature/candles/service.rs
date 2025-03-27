use std::sync::Arc;

use super::repository::CandlesRepositoryTrait;

#[derive(Clone)]
pub struct CandlesService<T: CandlesRepositoryTrait + Send + Sync + 'static> {
    candles_service: Arc<T>,
}

impl<T: CandlesRepositoryTrait + Send + Sync + 'static> CandlesService<T> {
    pub fn new_candles_services(candles_service: Arc<T>) -> Self {
        Self { candles_service }
    }
}
