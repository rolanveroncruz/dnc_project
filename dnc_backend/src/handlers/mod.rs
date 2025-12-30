
pub mod boiler;
pub mod user_roles_permissions;
mod structs;
mod request_parts;
mod middlewares;
mod helpers;
mod api;

pub use api::dental_services::get_dental_services;
pub use api::clinic_capabilities::get_clinic_capabilities;
pub use api::users::get_users;
pub use structs::{AuthUser, Claims, JwtConfig, ListQuery, PageResponse};

pub use user_roles_permissions::{LoginRequest, LoginResponse};

pub use middlewares::{inject_jwt_config, require_jwt};
pub use boiler::WhoAmIResponse;