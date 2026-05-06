use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ColumnTrait, EntityTrait, JoinType, QueryFilter, QueryOrder, QuerySelect, RelationTrait};
use sea_orm::prelude::{Date, DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    AppState,
    entities::{
        master_list_member,
        endorsement,
        endorsement_company,
        master_list,
    },

};

#[derive(Debug, Serialize)]
pub struct MasterListMemberResponse {
    pub id: i32,
    pub endorsement_id: i32,
    pub master_list_id: Option<i32>,
    pub account_number: String,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: String,
    pub email_address: Option<String>,
    pub mobile_number: Option<String>,
    pub birth_date: Option<Date>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateMasterListMemberRequest {
    pub endorsement_id: i32,
    pub master_list_id: Option<i32>,
    pub account_number: String,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: String,
    pub email_address: Option<String>,
    pub mobile_number: Option<String>,
    pub birth_date: Option<Date>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct PatchMasterListMemberRequest {
    pub endorsement_id: Option<i32>,
    pub master_list_id: Option<Option<i32>>,
    pub account_number: Option<String>,
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub email_address: Option<Option<String>>,
    pub mobile_number: Option<Option<String>>,
    pub birth_date: Option<Option<Date>>,
    pub is_active: Option<bool>,
}

// region: get_master_list_members_for_endorsement()

#[derive(Debug, Serialize, sea_orm::FromQueryResult)]
pub struct MasterListMemberForEndorsementResponse {
    pub endorsement_company_name: String,
    pub agreement_corp_number: Option<String>,

    pub id: i32,
    pub endorsement_id: i32,
    pub master_list_id: Option<i32>,
    pub account_number: String,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: String,
    pub email_address: Option<String>,
    pub mobile_number: Option<String>,
    pub birth_date: Option<Date>,
    pub is_active: bool,
}

#[instrument(skip(state), err(Debug))]
pub async fn get_master_list_members_for_endorsement(
    State(state): State<AppState>,
    Path(endorsement_id): Path<i32>,
) -> Result<Json<Vec<MasterListMemberForEndorsementResponse>>, (StatusCode, String)> {
    let rows = master_list_member::Entity::find()
        .filter(master_list_member::Column::EndorsementId.eq(endorsement_id))
        .join(
            JoinType::InnerJoin,
            master_list_member::Relation::Endorsement.def(),
        )
        .join(
            JoinType::InnerJoin,
            endorsement::Relation::EndorsementCompany.def(),
        )
        .select_only()
        .column_as(
            endorsement_company::Column::Name,
            "endorsement_company_name",
        )
        .column_as(
            endorsement::Column::AgreementCorpNumber,
            "agreement_corp_number",
        )
        .column(master_list_member::Column::Id)
        .column(master_list_member::Column::EndorsementId)
        .column(master_list_member::Column::MasterListId)
        .column(master_list_member::Column::AccountNumber)
        .column(master_list_member::Column::LastName)
        .column(master_list_member::Column::FirstName)
        .column(master_list_member::Column::MiddleName)
        .column(master_list_member::Column::EmailAddress)
        .column(master_list_member::Column::MobileNumber)
        .column(master_list_member::Column::BirthDate)
        .column(master_list_member::Column::IsActive)
        .into_model::<MasterListMemberForEndorsementResponse>()
        .all(&state.db)
        .await
        .map_err(internal_error)?;

    Ok(Json(rows))
}

fn internal_error(err: sea_orm::DbErr) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

// endregion: get_master_list_members_for_endorsement()

// region: get_master_lists_with_members_for_endorsement()

#[derive(Debug, Serialize)]
pub struct MasterListsForEndorsementResponse {
    pub endorsement_company_name: String,
    pub agreement_corp_number: Option<String>,
    pub master_lists: Vec<MasterListForEndorsementResponse>,
}

#[derive(Debug, Serialize)]
pub struct MasterListForEndorsementResponse {
    pub master_list_id: Option<i32>,
    pub file_name: Option<String>,
    pub date_uploaded: Option<DateTimeWithTimeZone>,
    pub members: Vec<MasterListMemberForEndorsementMemberResponse>,
}

#[derive(Debug, Serialize)]
pub struct MasterListMemberForEndorsementMemberResponse {
    pub id: i32,
    pub endorsement_id: i32,
    pub master_list_id: Option<i32>,
    pub account_number: String,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: String,
    pub email_address: Option<String>,
    pub mobile_number: Option<String>,
    pub birth_date: Option<Date>,
    pub is_active: bool,
}

#[derive(Debug, sea_orm::FromQueryResult)]
struct MasterListMemberForEndorsementRow {
    pub endorsement_company_name: String,
    pub agreement_corp_number: Option<String>,

    pub master_list_file_name: Option<String>,
    pub master_list_date_uploaded: Option<DateTimeWithTimeZone>,

    pub id: i32,
    pub endorsement_id: i32,
    pub master_list_id: Option<i32>,
    pub account_number: String,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: String,
    pub email_address: Option<String>,
    pub mobile_number: Option<String>,
    pub birth_date: Option<Date>,
    pub is_active: bool,
}

#[instrument(skip(state), err(Debug))]
pub async fn get_master_lists_with_members_for_endorsement(
    State(state): State<AppState>,
    Path(endorsement_id): Path<i32>,
) -> Result<Json<MasterListsForEndorsementResponse>, (StatusCode, String)> {
    let rows = master_list_member::Entity::find()
        .filter(master_list_member::Column::EndorsementId.eq(endorsement_id))
        .join(
            JoinType::InnerJoin,
            master_list_member::Relation::Endorsement.def(),
        )
        .join(
            JoinType::LeftJoin,
            master_list_member::Relation::MasterList.def(),
        )
        .join(
            JoinType::InnerJoin,
            endorsement::Relation::EndorsementCompany.def(),
        )
        .select_only()
        .column_as(
            endorsement_company::Column::Name,
            "endorsement_company_name",
        )
        .column_as(
            endorsement::Column::AgreementCorpNumber,
            "agreement_corp_number",
        )
        .column_as(
            master_list::Column::FileName,
            "master_list_file_name",
        )
        .column_as(
            master_list::Column::UploadDate,
            "master_list_date_uploaded",
        )
        .column(master_list_member::Column::Id)
        .column(master_list_member::Column::EndorsementId)
        .column(master_list_member::Column::MasterListId)
        .column(master_list_member::Column::AccountNumber)
        .column(master_list_member::Column::LastName)
        .column(master_list_member::Column::FirstName)
        .column(master_list_member::Column::MiddleName)
        .column(master_list_member::Column::EmailAddress)
        .column(master_list_member::Column::MobileNumber)
        .column(master_list_member::Column::BirthDate)
        .column(master_list_member::Column::IsActive)
        .order_by_asc(master_list_member::Column::MasterListId)
        .order_by_asc(master_list_member::Column::LastName)
        .order_by_asc(master_list_member::Column::FirstName)
        .into_model::<MasterListMemberForEndorsementRow>()
        .all(&state.db)
        .await
        .map_err(internal_error)?;

    let mut endorsement_company_name = String::new();
    let mut agreement_corp_number = None;
    let mut master_lists: Vec<MasterListForEndorsementResponse> = Vec::new();

    for row in rows {
        if endorsement_company_name.is_empty() {
            endorsement_company_name = row.endorsement_company_name.clone();
            agreement_corp_number = row.agreement_corp_number.clone();
        }

        let member = MasterListMemberForEndorsementMemberResponse {
            id: row.id,
            endorsement_id: row.endorsement_id,
            master_list_id: row.master_list_id,
            account_number: row.account_number,
            last_name: row.last_name,
            first_name: row.first_name,
            middle_name: row.middle_name,
            email_address: row.email_address,
            mobile_number: row.mobile_number,
            birth_date: row.birth_date,
            is_active: row.is_active,
        };

        match master_lists
            .iter()
            .position(|master_list| master_list.master_list_id == row.master_list_id)
        {
            Some(index) => {
                master_lists[index].members.push(member);
            }
            None => {
                master_lists.push(MasterListForEndorsementResponse {
                    master_list_id: row.master_list_id,
                    file_name: row.master_list_file_name,
                    date_uploaded: row.master_list_date_uploaded,
                    members: vec![member],
                });
            }
        }
    }

    Ok(Json(MasterListsForEndorsementResponse {
        endorsement_company_name,
        agreement_corp_number,
        master_lists,
    }))

}

// endregion: get_master_lists_with_members_for_endorsement()