use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

use axum::{extract::{Path, State}, http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait,
              JoinType, QueryFilter, QueryOrder, QuerySelect, Set,
};

use crate::entities::{
    acc_reconciliation,
    dental_service, dentist, endorsement,
    endorsement_company,
    master_list_member, tooth_service_type,
    tooth_surface, verification,
};
use chrono::{Utc};

// region: Get Done Verifications
#[derive(Debug, Serialize, Deserialize)]
pub struct DoneVerificationResponse {
    pub id: i32,
    pub date_created: DateTimeWithTimeZone,
    pub dentist_name: String,
    pub member_name: String,
    pub dental_service_name: String,
    pub agreement_corp_number: Option<String>,
    pub company_name: String,
    pub date_service_performed: Option<Date>,
    pub tooth_id: Option<String>,
    pub tooth_surface_name: Option<String>,
    pub tooth_service_type_name: Option<String>,
    pub approval_code: Option<String>,
    pub approval_date: Option<DateTimeWithTimeZone>,
    pub is_reconciled: Option<bool>,
    pub reconciled_by: Option<String>,
    pub reconciliation_date: Option<DateTimeWithTimeZone>,
}

use sea_orm::FromQueryResult;
use crate::AppState;
use crate::handlers::AuthUser;

#[derive(Debug, FromQueryResult)]
struct DoneVerificationRow {
    pub id: i32,
    pub date_created: DateTimeWithTimeZone,

    pub dentist_first_name: String,
    pub dentist_last_name: String,

    pub member_first_name: String,
    pub member_last_name: String,
    pub member_middle_name: Option<String>,

    pub dental_service_name: String,
    pub agreement_corp_number: Option<String>,
    pub company_name: String,
    pub date_service_performed: Option<Date>,
    pub tooth_id: Option<String>,
    pub tooth_surface_name: Option<String>,
    pub tooth_service_type_name: Option<String>,
    pub approval_code: Option<String>,
    pub approval_date: Option<DateTimeWithTimeZone>,
    pub is_reconciled: Option<bool>,
    pub reconciled_by: Option<String>,
    pub reconciliation_date: Option<DateTimeWithTimeZone>,
}


pub async fn get_done_verifications(
    State(state): State<AppState>,
) -> Result<Json<Vec<DoneVerificationResponse>>, (StatusCode, String)> {
    let db = &state.db;
    let rows: Vec<DoneVerificationRow> = verification::Entity::find()
        .filter(verification::Column::StatusId.eq(99))
        .join(JoinType::InnerJoin, verification::Relation::Dentist.def())
        .join(JoinType::InnerJoin, verification::Relation::MasterListMember.def())
        .join(JoinType::InnerJoin, verification::Relation::DentalService.def())
        .join(JoinType::InnerJoin, master_list_member::Relation::Endorsement.def())
        .join(JoinType::InnerJoin, endorsement::Relation::EndorsementCompany.def())
        .join(JoinType::LeftJoin, verification::Relation::ToothSurface.def())
        .join(JoinType::LeftJoin, verification::Relation::ToothServiceType.def())
        .select_only()
        .column(verification::Column::Id)
        .column(verification::Column::DateCreated)
        .column(verification::Column::DateServicePerformed)
        .column(verification::Column::ToothId)
        .column(verification::Column::ApprovalCode)
        .column(verification::Column::ApprovalDate)
        .column(verification::Column::IsReconciled)
        .column(verification::Column::ReconciledBy)
        .column(verification::Column::ReconciliationDate)
        .column_as(dentist::Column::GivenName, "dentist_first_name")
        .column_as(dentist::Column::LastName, "dentist_last_name")
        .column_as(master_list_member::Column::FirstName, "member_first_name")
        .column_as(master_list_member::Column::LastName, "member_last_name")
        .column_as(master_list_member::Column::MiddleName, "member_middle_name")
        .column_as(dental_service::Column::Name, "dental_service_name")
        .column_as(endorsement::Column::AgreementCorpNumber, "agreement_corp_number")
        .column_as(endorsement_company::Column::Name, "company_name")
        .column_as(tooth_surface::Column::Name, "tooth_surface_name")
        .column_as(tooth_service_type::Column::Name, "tooth_service_type_name")
        .order_by_desc(verification::Column::DateCreated)
        .into_model::<DoneVerificationRow>()
        .all(db)
        .await
        .map_err(internal_error)?;

    let result = rows
        .into_iter()
        .map(|row| DoneVerificationResponse {
            id: row.id,
            date_created: row.date_created,
            dentist_name: format!("{}, {}", row.dentist_last_name, row.dentist_first_name),
            member_name: build_member_name(
                &row.member_last_name,
                &row.member_first_name,
                row.member_middle_name.as_deref(),
            ),
            dental_service_name: row.dental_service_name,
            agreement_corp_number: row.agreement_corp_number,
            company_name: row.company_name,
            date_service_performed: row.date_service_performed,
            tooth_id: row.tooth_id,
            tooth_surface_name: row.tooth_surface_name,
            tooth_service_type_name: row.tooth_service_type_name,
            approval_code: row.approval_code,
            approval_date: row.approval_date,
            is_reconciled: row.is_reconciled,
            reconciled_by: row.reconciled_by,
            reconciliation_date: row.reconciliation_date,
        })
        .collect();

    Ok(Json(result))
}
fn build_member_name(last: &str, first: &str, middle: Option<&str>) -> String {
    match middle {
        Some(m) if !m.trim().is_empty() => format!("{}, {} {}", last, first, m),
        _ => format!("{}, {}", last, first),
    }
}

fn internal_error<E: std::fmt::Display>(err: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

// endregion: Get Done Verifications


// region: Reconcile Verification


pub async fn reconcile_verification(
    State(state): State<AppState>,
    user: AuthUser,
    Path(verification_id): Path<i32>,
) -> Result<Json<DoneVerificationResponse>, (StatusCode, String)> {
    let db = &state.db;

    let model = verification::Entity::find_by_id(verification_id)
        .one(db)
        .await
        .map_err(internal_error)?
        .ok_or((
            StatusCode::NOT_FOUND,
            format!("Verification {} not found", verification_id),
        ))?;

    let mut active_model: verification::ActiveModel = model.into();

    active_model.is_reconciled = Set(Some(true));
    active_model.reconciled_by = Set(Some(user.claims.email));
    active_model.reconciliation_date = Set(Some(Utc::now().fixed_offset()));

    let updated = active_model.update(db).await.map_err(internal_error)?;

    let row: DoneVerificationRow = verification::Entity::find()
        .filter(verification::Column::Id.eq(updated.id))
        .join(JoinType::InnerJoin, verification::Relation::Dentist.def())
        .join(JoinType::InnerJoin, verification::Relation::MasterListMember.def())
        .join(JoinType::InnerJoin, verification::Relation::DentalService.def())
        .join(JoinType::LeftJoin, verification::Relation::ToothSurface.def())
        .join(JoinType::LeftJoin, verification::Relation::ToothServiceType.def())
        .select_only()
        .column(verification::Column::Id)
        .column(verification::Column::DateCreated)
        .column(verification::Column::DateServicePerformed)
        .column(verification::Column::ToothId)
        .column(verification::Column::ApprovalCode)
        .column(verification::Column::ApprovalDate)
        .column(verification::Column::IsReconciled)
        .column(verification::Column::ReconciledBy)
        .column(verification::Column::ReconciliationDate)
        .column_as(dentist::Column::GivenName, "dentist_first_name")
        .column_as(dentist::Column::LastName, "dentist_last_name")
        .column_as(master_list_member::Column::FirstName, "member_first_name")
        .column_as(master_list_member::Column::LastName, "member_last_name")
        .column_as(master_list_member::Column::MiddleName, "member_middle_name")
        .column_as(dental_service::Column::Name, "dental_service_name")
        .column_as(tooth_surface::Column::Name, "tooth_surface_name")
        .column_as(tooth_service_type::Column::Name, "tooth_service_type_name")
        .into_model::<DoneVerificationRow>()
        .one(db)
        .await
        .map_err(internal_error)?
        .ok_or((
            StatusCode::NOT_FOUND,
            format!("Verification {} not found after update", verification_id),
        ))?;

        let response = DoneVerificationResponse {
            id: row.id,
            date_created: row.date_created,
            dentist_name: format!("{}, {}", row.dentist_last_name, row.dentist_first_name),
            member_name: build_member_name(
                &row.member_last_name,
                &row.member_first_name,
                row.member_middle_name.as_deref(),
            ),
            dental_service_name: row.dental_service_name,
            agreement_corp_number: row.agreement_corp_number,
            company_name: row.company_name,
            date_service_performed: row.date_service_performed,
            tooth_id: row.tooth_id,
            tooth_surface_name: row.tooth_surface_name,
            tooth_service_type_name: row.tooth_service_type_name,
            approval_code: row.approval_code,
            approval_date: row.approval_date,
            is_reconciled: row.is_reconciled,
            reconciled_by: row.reconciled_by,
            reconciliation_date: row.reconciliation_date,
        };

        Ok(Json(response))
}
// endregion: Reconcile Verification


// region: Add Accomplishment Reconciliation
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccReconciliationRequest {
    pub dentist_id: i32,
    pub member_id: i32,
    pub dental_service_id: i32,
    pub date_service_performed: Option<Date>,
    pub approval_code: Option<String>,

    pub tooth_id: Option<String>,
    pub tooth_service_type_id: Option<i32>,
    pub tooth_surface_id: Option<i32>,
}

pub async fn create_acc_reconciliation(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreateAccReconciliationRequest>,
) -> Result<Json<acc_reconciliation::Model>, (StatusCode, String)> {
    let db = &state.db;

    let new_reconciliation = acc_reconciliation::ActiveModel {
        id: Default::default(),
        date_created:  Set(Utc::now().fixed_offset()),
        created_by:  Set(user.claims.email),

        dentist_id:  Set(payload.dentist_id),
        member_id:  Set(payload.member_id),
        dental_service_id:  Set(payload.dental_service_id),
        date_service_performed:  Set(payload.date_service_performed),

        approved_by:  Set(None),
        approval_date:  Set(None),
        approval_code:  Set(payload.approval_code),

        tooth_id:  Set(payload.tooth_id),
        tooth_service_type_id: Set(payload.tooth_service_type_id),
        tooth_surface_id:  Set(payload.tooth_surface_id),
    };

    let inserted = new_reconciliation
        .insert(db)
        .await
        .map_err(internal_error)?;

    Ok(Json(inserted))
}

// endregion: Add Accomplishment Reconciliation
