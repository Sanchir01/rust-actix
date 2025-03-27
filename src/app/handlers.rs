use super::services::Services;
use crate::feature::{
    candles::handler::CandlesHandler,
    user::{
        handler::UserHandler,
        repository::UserRepositoryTrait,
        service::{UserService, UserServiceTrait},
    },
};
use std::sync::Arc;

#[derive(Clone)]
pub struct Handlers {
    pub users_handler: Arc<UserHandler>,
    pub candles_handler: Arc<CandlesHandler>,
}

impl Handlers {
    pub fn new_handlers(services: Arc<Services>) -> Self {
        Self {
            users_handler: Arc::new(UserHandler::new(services.users_service.clone())),
            candles_handler: Arc::new(CandlesHandler::new(services.candles_service.clone())),
        }
    }
}
