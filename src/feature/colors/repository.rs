use super::entity::ColorsStruct;
use mockall::{automock, predicate::*};
use sqlx::{Pool, Postgres, query_scalar};
use uuid::Uuid;

#[cfg_attr(test, automock)]
pub trait ColorRepositoryTrait {
    async fn get_all_color(&self) -> Result<Vec<ColorsStruct>, sqlx::Error>;
    async fn create_color_repo(
        &self,
        title: &str,
        slug: &str,
        version: i64,
    ) -> Result<Uuid, sqlx::Error>;
}

#[derive(Clone)]
pub struct ColorRepository {
    color_repo: Pool<Postgres>,
}

impl ColorRepository {
    pub fn new_color_repository(color_repo: Pool<Postgres>) -> Self {
        Self { color_repo }
    }
}

impl ColorRepositoryTrait for ColorRepository {
    async fn get_all_color(&self) -> Result<Vec<ColorsStruct>, sqlx::Error> {
        let query = r#"
            SELECT id, title,slug,version FROM public.colors
        "#;
        let candles = sqlx::query_as(query)
            .fetch_all(&self.color_repo)
            .await
            .map_err(|err| {
                eprintln!("Error fetching candles: {:?}", err);
                err
            })?;

        Ok(candles)
    }
    async fn create_color_repo(
        &self,
        title: &str,
        slug: &str,
        version: i64,
    ) -> Result<Uuid, sqlx::Error> {
        let query = r#"
            INSERT INTO colors (title, slug, version) VALUES ($1,$2,$3) RETURNING id 
        "#;

        let colors = query_scalar(query)
            .bind(title)
            .bind(slug)
            .bind(version)
            .fetch_one(&self.color_repo)
            .await
            .map_err(|err| {
                eprintln!("error creating: {:?}", err);
                err
            })?;
        Ok(colors)
    }
}
