
pub mod boiler;
pub mod login;
mod structs;
mod request_parts;
mod middlewares;
mod helpers;
mod api;

pub use api::dental_services::get_dental_services;
pub use api::dental_service_type::get_dental_service_types;
pub use api::clinic_capabilities::get_clinic_capabilities;
pub use api::users::get_users;
pub use api::roles::get_roles;
pub use api::role_permission::get_role_permissions;
pub use api::data_objects::get_data_objects;
pub use structs::{AuthUser, Claims, JwtConfig, ListQuery, PageResponse, AppError};

pub use login::{LoginRequest, LoginResponse};

pub use middlewares::{inject_jwt_config, require_jwt};
pub use boiler::WhoAmIResponse;
pub use api::hmo::{get_hmos, get_hmo_by_id};