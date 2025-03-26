use std::sync::Arc;

use crate::feature::{
    candles::service::CandlesService,
    user::{repository::UserRepository, service::UserService},
};

use super::repositories::Repositories;

#[derive(Clone)]
pub struct Services {
    pub users_service: Arc<UserService<UserRepository>>,
    pub candles_service: Arc<CandlesService>,
}

impl Services {
    pub fn new_sevices(repositories: Arc<Repositories>) -> Self {
        Self {
            users_service: Arc::new(UserService::new_user_services(
                repositories.user_repository.clone(),
            )),
            candles_service: Arc::new(CandlesService::new_candles_services(
                repositories.candles_repository.clone(),
            )),
        }
    }
}
