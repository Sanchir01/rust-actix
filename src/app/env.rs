#[derive(Clone)]
pub struct Env {
    pub database: Arc<Database>,
}

impl Env {
    pub async fn NewEnv() -> Result<Self> {
        Ok(Self {})
    }
}
