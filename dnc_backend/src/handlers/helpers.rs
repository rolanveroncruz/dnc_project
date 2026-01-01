use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QuerySelect};
use crate::entities::{data_object, permission, role_permission};
use crate::entities::sea_orm_active_enums::PermissionActionEnum;

pub async fn role_has_permission_by_data_object_name(
    db: &DatabaseConnection,
    role_id: i32,
    data_object_name: &str,
    action:PermissionActionEnum,
)->Result<bool, DbErr> {
    let found = permission::Entity::find()
        .select_only()
        .column(permission::Column::Id)
        .inner_join(role_permission::Entity)
        .inner_join(data_object::Entity)
        .filter(role_permission::Column::RoleId.eq(role_id))
        .filter(permission::Column::Action.eq(action))
        .filter(data_object::Column::Name.eq(data_object_name))
        .into_tuple::<i32>()
        .one(db)
        .await?
        .is_some();

    Ok (found)
}

