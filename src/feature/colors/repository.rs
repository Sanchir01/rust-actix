use super::entity::ColorsStruct;
use mockall::{automock, predicate::*};
use sqlx::{Pool, Postgres};

#[cfg_attr(test, automock)]
pub trait ColorRepositoryTrait {
    async fn get_all_color(&self) -> Result<Vec<ColorsStruct>, sqlx::Error>;
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
}
