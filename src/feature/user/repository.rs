use crate::feature::user::entity::User;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct UserRepository {
    user_repo: Pool<Postgres>,
}

impl UserRepository {
    pub fn new_user_repository(user_repo: Pool<Postgres>) -> Self {
        Self { user_repo }
    }
    pub async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let query = r#"
            SELECT id, title, slug
            FROM users
        "#;
        let users = sqlx::query_as(query)
            .fetch_all(&self.user_repo)
            .await
            .map_err(|e| {
                eprintln!("Error fetching users: {:?}", e);
                e
            })?;
        Ok(users)
    }
    pub async fn create_user(&self, title: &str, slug: &str) -> Result<String, sqlx::Error> {
        let query = r#"
            INSERT  INTO users (title, slug)
            VALUES ($1, $2) RETURNING id
            "#;
        let user: String = sqlx::query_scalar(query)
            .bind(title)
            .bind(slug)
            .fetch_one(&self.user_repo)
            .await
            .map_err(|e| {
                eprintln!("Error fetching users: {:?}", e);
                e
            })?;
        Ok(user)
    }
}
