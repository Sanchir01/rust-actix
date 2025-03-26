use crate::feature::user::{entity::User, handler::UserResponse};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(crate::feature::user::handler::get_users),
    components(
        schemas(User,UserResponse)
    ),
    tags(
        (name = "users", description = "User management API")
    ),
)]
struct ApiDoc;

pub fn setup_swagger() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi())
}
