use std::sync::Arc;

use slug::slugify;
use uuid::Uuid;

use super::{
    entity::ColorsStruct,
    repository::{ColorRepository, ColorRepositoryTrait},
};

pub trait ColorServiceTrait {
    async fn get_all_color_service(&self) -> Result<Vec<ColorsStruct>, sqlx::Error>;
    async fn create_color_service(&self, title: &str, version: i64) -> Result<Uuid, sqlx::Error>;
}

#[derive(Clone)]
pub struct ColorService {
    color_repo: Arc<ColorRepository>,
}

impl ColorService {
    pub fn new_color_services(color_repo: Arc<ColorRepository>) -> Self {
        Self { color_repo }
    }
}

impl ColorServiceTrait for ColorService {
    async fn get_all_color_service(&self) -> Result<Vec<ColorsStruct>, sqlx::Error> {
        self.color_repo.get_all_color().await
    }
    async fn create_color_service(&self, title: &str, version: i64) -> Result<Uuid, sqlx::Error> {
        let title_slug = slugify(title);
        let color_id = self
            .color_repo
            .create_color_repo(title, &title_slug, version)
            .await?;
        Ok(color_id)
    }
}
