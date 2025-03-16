use sqlx::{postgres::PgPool};
use crate::feature::user::entity::User;

pub struct UserRepository {
    user_repo: PgPool,
}

impl UserRepository {
    pub fn new(user_repo: PgPool) -> Self {
        Self { user_repo }
    }
    pub async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let query = r#"
            SELECT id, name, slug
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
}