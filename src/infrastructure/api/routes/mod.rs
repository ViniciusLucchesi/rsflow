pub mod user_routes;
pub mod group_routes;
pub mod user_group_routers;


use axum::response::IntoResponse;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use std::sync::Arc;
use crate::core::interfaces::database::DatabaseError;
use crate::core::services::group_service::GroupService;
use crate::core::services::user_service::UserService;
use crate::core::services::user_group_service::UserGroupService;
use crate::adapters::controllers::user_controller::{CreateUserRequest, UserResponse};
use crate::adapters::controllers::group_controller::{CreateGroupRequest, GroupResponse};
use crate::adapters::controllers::user_group_controller::{CreateUserGroupRequest, UserGroupResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        user_routes::create_user,
        user_routes::get_user_by_id,
        user_routes::get_all_users,
        group_routes::create_group,
        group_routes::get_group_by_id,
        group_routes::get_all_groups,
        user_group_routers::create_user_group,
        user_group_routers::get_user_group_by_id,
        user_group_routers::get_user_group_by_user_id,
        user_group_routers::get_user_group_by_group_id,
        user_group_routers::get_all_user_groups,
    ),
    components(
        schemas(
            CreateUserRequest,
            UserResponse,
            CreateGroupRequest,
            GroupResponse,
            CreateUserGroupRequest,
            UserGroupResponse,
        )
    ),
    tags(
        (name = "Users", description = "Operações relacionadas a usuários"),
        (name = "Groups", description = "Operações relacionadas a grupos"),
        (name = "UserGroups", description = "Associações entre usuários e grupos")
    )
)]
pub struct ApiDoc;


pub fn build_routes(user: Arc<UserService>, group: Arc<GroupService>, user_group: Arc<UserGroupService>) -> Router {
    let openapi = ApiDoc::openapi();
    Router::new()
        .merge(SwaggerUi::new("/swagger").url("/api-doc/openapi.json", openapi))
        .nest("/users", user_routes::build_routes(user))
        .nest("/groups", group_routes::build_routes(group))
        .nest("/user-groups", user_group_routers::build_routes(user_group))
}


struct ErrorResponse {
    message: String,
}

impl From<DatabaseError> for ErrorResponse {
    fn from(error: DatabaseError) -> Self {
        ErrorResponse {
            message: error.to_string(),
        }
    }
}