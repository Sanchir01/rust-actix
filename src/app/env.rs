use sqlx::PgPool;

pub struct Env {
    pub dbpool: PgPool,
}

impl Env {}

