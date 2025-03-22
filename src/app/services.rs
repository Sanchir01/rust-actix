use std::sync::Arc;

use crate::feature::user::service::UserService;

use super::repositories::Repositories;

#[derive(Clone)]
pub struct Services {
    pub users_service: Arc<UserService>,
}

impl Services {
    pub fn new_sevices(repositories: Arc<Repositories>) -> Self {
        Self {
            users_service: Arc::new(UserService::new_user_services(
                repositories.user_repository.clone(),
            )),
        }
    }
}
