use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, FromQueryResult, JoinType, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set};
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


// region: Save Member Name for Company

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveMemberNameRequest {
    pub name: String,
    pub account_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveMemberNameResponse {
    pub master_list_member_id: i32,
}

#[axum::debug_handler(state = AppState)]
pub async fn save_member_name_for_company(
    State(state): State<AppState>,
    Path(company_id): Path<i32>,
    Json(payload): Json<SaveMemberNameRequest>,
) -> Result<Json<SaveMemberNameResponse>, (StatusCode, String)> {

    let db: &DatabaseConnection = &state.db;

    // ✅ Get the active endorsement for this company
    let endorsement = endorsement::Entity::find()
        .filter(endorsement::Column::EndorsementCompanyId.eq(company_id))
        .filter(endorsement::Column::IsActive.eq(true))
        .order_by_desc(endorsement::Column::DateEnd)
        .order_by_desc(endorsement::Column::Id)
        .one(db)
        .await
        .map_err(internal_error)?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                format!("No active endorsement found for company_id {}", company_id),
            )
        })?;

    // ✅ Parse: "last_name, first_name middle_name"
    let (last_name, first_and_middle) = payload
        .name
        .split_once(',')
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                "Name must be formatted as: last_name, first_name middle_name".to_string(),
            )
        })?;

    let last_name = last_name.trim().to_string();

    let name_parts: Vec<&str> = first_and_middle
        .trim()
        .split_whitespace()
        .collect();

    if name_parts.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "First name is required after the comma".to_string(),
        ));
    }

    // ✅ Last word is middle_name, the rest is first_name
    let middle_name = if name_parts.len() > 1 {
        name_parts[name_parts.len() - 1].to_string()
    } else {
        "".to_string()
    };

    let first_name = if name_parts.len() > 1 {
        name_parts[..name_parts.len() - 1].join(" ")
    } else {
        name_parts[0].to_string()
    };

    // ✅ If account_number is null/empty, use "N/A"
    let account_number = payload
        .account_number
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "N/A".to_string());

    // ✅ Insert new master_list_member
    let new_member = master_list_member::ActiveModel {
        id: Default::default(),
        endorsement_id: Set(endorsement.id),
        master_list_id: Set(None),
        account_number: Set(account_number),
        last_name: Set(last_name),
        first_name: Set(first_name),
        middle_name: Set(middle_name),
        email_address: Set(None),
        mobile_number: Set(None),
        birth_date: Set(None),
        is_active: Set(true),
        last_edited_by: Set(None),
        last_edited_date: Set(chrono::Utc::now().into()),
    };

    let insert_result = master_list_member::Entity::insert(new_member)
        .exec(db)
        .await
        .map_err(internal_error)?;

    Ok(Json(SaveMemberNameResponse {
        master_list_member_id: insert_result.last_insert_id,
    }))
}

// endregion: Save Member Name For Company