use std::sync::Arc;

use uuid::Uuid;

use super::{entity::ColorsStruct, repository::ColorRepository};

pub trait ColorServiceTrait {
    async fn get_all_candles(&self) -> Result<Vec<ColorsStruct>, sqlx::Error>;
    async fn create_candle(
        &self,
        title: &str,
        price: i64,
        version: i64,
        color_id: Uuid,
    ) -> Result<Uuid, Box<dyn std::error::Error>>;
}

#[derive(Clone)]
pub struct ColorService {
    color_service: Arc<ColorRepository>,
}

impl ColorService {
    pub fn new_color_services(color_service: Arc<ColorRepository>) -> Self {
        Self { color_service }
    }
}
