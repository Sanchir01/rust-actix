use std::sync::Arc;

use uuid::Uuid;

use super::repository::CandlesRepository;

#[derive(Clone)]
pub struct CandlesService {
    candles_service: Arc<CandlesRepository>,
}

impl CandlesService {
    pub fn new_candles_services(candles_service: Arc<CandlesRepository>) -> Self {
        Self { candles_service }
    }
}
