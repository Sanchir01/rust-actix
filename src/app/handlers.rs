use super::services::Services;
use crate::feature::{
    candles::handler::CandlesHandler, colors::handler::ColorHandler, user::handler::UserHandler,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct Handlers {
    pub users_handler: Arc<UserHandler>,
    pub candles_handler: Arc<CandlesHandler>,
    pub color_handler: Arc<ColorHandler>,
}

impl Handlers {
    pub fn new_handlers(services: Arc<Services>) -> Self {
        Self {
            users_handler: Arc::new(UserHandler::new(services.users_service.clone())),
            candles_handler: Arc::new(CandlesHandler::new(services.candles_service.clone())),
            color_handler: Arc::new(ColorHandler::new(services.color_service.clone())),
        }
    }
}
