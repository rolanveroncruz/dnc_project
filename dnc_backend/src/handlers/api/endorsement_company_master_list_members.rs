use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, FromQueryResult, JoinType, QueryFilter, QueryOrder, QuerySelect, RelationTrait};
use serde::{Deserialize, Serialize};

use crate::{AppState, entities::{endorsement, master_list_member}};

// region: Get All Member Names From Company

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberNameResponse {
    pub id: i32,
    pub full_name: String,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: String,
}

#[derive(Debug, FromQueryResult)]
struct MemberNameRow {
    pub id: i32,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: String,
}

pub async fn get_all_member_names_from_company(
    State(state): State<AppState>,
    Path(company_id): Path<i32>,
) -> Result<Json<Vec<MemberNameResponse>>, (StatusCode, String)> {
    let db: &DatabaseConnection = &state.db;

    let rows = master_list_member::Entity::find()
        .join(
            JoinType::InnerJoin,
            master_list_member::Relation::Endorsement.def(),
        )
        .filter(endorsement::Column::EndorsementCompanyId.eq(company_id))
        .filter(master_list_member::Column::IsActive.eq(true))
        .select_only()
        .column(master_list_member::Column::Id)
        .column(master_list_member::Column::LastName)
        .column(master_list_member::Column::FirstName)
        .column(master_list_member::Column::MiddleName)
        .order_by_asc(master_list_member::Column::LastName)
        .order_by_asc(master_list_member::Column::FirstName)
        .order_by_asc(master_list_member::Column::MiddleName)
        .into_model::<MemberNameRow>()
        .all(db)
        .await
        .map_err(internal_error)?;

    let response = rows
        .into_iter()
        .map(|row| {
            let full_name = format!(
                "{}, {} {}",
                row.last_name.trim(),
                row.first_name.trim(),
                row.middle_name.trim()
            )
                .trim()
                .to_string();

            MemberNameResponse {
                id: row.id,
                full_name,
                last_name: row.last_name,
                first_name: row.first_name,
                middle_name: row.middle_name,
            }
        })
        .collect();

    Ok(Json(response))
}

fn internal_error(err: DbErr) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
// endregion: Get All Member Names From Company