use std::sync::Arc;

use crate::feature::user::handler::UserHandler;

use super::services::Services;

#[derive(Clone)]
pub struct Handlers {
    pub users_handler: Arc<UserHandler>,
}

impl Handlers {
    pub fn new_handlers(services: Arc<Services>) -> Self {
        Self {
            users_handler: Arc::new(UserHandler::new(services.users_service.clone())),
        }
    }
}
