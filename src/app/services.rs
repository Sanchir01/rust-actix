use std::sync::Arc;

use crate::feature::{
    candles::service::CandlesService, colors::service::ColorService, user::service::UserService,
};

use super::repositories::Repositories;

#[derive(Clone)]
pub struct Services {
    pub users_service: Arc<UserService>,
    pub candles_service: Arc<CandlesService>,
    pub color_service: Arc<ColorService>,
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
            color_service: Arc::new(ColorService::new_color_services(
                repositories.color_repository.clone(),
            )),
        }
    }
}
