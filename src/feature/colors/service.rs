use std::sync::Arc;

use super::{
    entity::ColorsStruct,
    repository::{ColorRepository, ColorRepositoryTrait},
};

pub trait ColorServiceTrait {
    async fn get_all_colors_service(&self) -> Result<Vec<ColorsStruct>, sqlx::Error>;
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

impl ColorServiceTrait for ColorService {
    async fn get_all_colors_service(&self) -> Result<Vec<ColorsStruct>, sqlx::Error> {
        self.color_service.get_all_color().await
    }
}
