use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use sea_orm::{
    ActiveModelTrait, EntityTrait, FromQueryResult, JoinType, PaginatorTrait,
    QueryOrder, QuerySelect, RelationTrait, Set,
};

use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::entities::{
    endorsement, endorsement_billing_period_type, endorsement_company, endorsement_type, hmo,
};

// If you have sea_orm::Decimal in your project already, this matches your entity.
// If your Decimal type is coming from a different crate, adjust here.
use rust_decimal::Decimal;

//
// ---- Paging
//

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<u64>,      // 1-based
    pub page_size: Option<u64>, // clamp server-side
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct PageResponse<T> {
    pub page: u64, // 1-based
    pub page_size: u64,
    pub total: u64,
    pub items: Vec<T>,
}

//
// ---- DTOs
//

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct EndorsementResponse {
    pub id: i32,
    pub hmo_id: i32,
    pub endorsement_company_id: i32,
    pub endorsement_type_id: i32,
    pub agreement_corp_number: Option<String>,
    pub date_start: sea_orm::prelude::Date,
    pub date_end: sea_orm::prelude::Date,
    pub endorsement_billing_period_type_id: i32,
    pub retainer_fee: Option<Decimal>,
    pub remarks: Option<String>,
    pub endorsement_method: Option<String>,
    pub is_active: bool,
}

impl From<endorsement::Model> for EndorsementResponse {
    fn from(m: endorsement::Model) -> Self {
        Self {
            id: m.id,
            hmo_id: m.hmo_id,
            endorsement_company_id: m.endorsement_company_id,
            endorsement_type_id: m.endorsement_type_id,
            agreement_corp_number: m.agreement_corp_number,
            date_start: m.date_start,
            date_end: m.date_end,
            endorsement_billing_period_type_id: m.endorsement_billing_period_type_id,
            retainer_fee: m.retainer_fee,
            remarks: m.remarks,
            endorsement_method: m.endorsement_method,
            is_active: m.is_active,
        }
    }
}

/// List row: base endorsement columns + joined names
#[allow(dead_code)]
#[derive(Debug, Serialize, FromQueryResult)]
pub struct EndorsementListRow {
    pub id: i32,
    pub hmo_id: i32,
    pub endorsement_company_id: i32,
    pub endorsement_type_id: i32,
    pub agreement_corp_number: Option<String>,
    pub date_start: sea_orm::prelude::Date,
    pub date_end: sea_orm::prelude::Date,
    pub endorsement_billing_period_type_id: i32,
    pub retainer_fee: Option<Decimal>,
    pub remarks: Option<String>,
    pub endorsement_method: Option<String>,
    pub is_active: bool,

    // joined fields
    pub hmo_name: Option<String>,
    pub company_name: Option<String>,
    pub type_name: Option<String>,
    pub billing_period_type_name: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CreateEndorsementRequest {
    pub hmo_id: i32,
    pub endorsement_company_id: i32,
    pub endorsement_type_id: i32,
    pub agreement_corp_number: Option<String>,
    pub date_start: sea_orm::prelude::Date,
    pub date_end: sea_orm::prelude::Date,
    pub endorsement_billing_period_type_id: i32,
    pub retainer_fee: Option<Decimal>,
    pub remarks: Option<String>,
    pub endorsement_method: Option<String>,
    pub is_active: bool,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct PatchEndorsementRequest {
    pub hmo_id: Option<i32>,
    pub endorsement_company_id: Option<i32>,
    pub endorsement_type_id: Option<i32>,
    pub agreement_corp_number: Option<Option<String>>,
    pub date_start: Option<sea_orm::prelude::Date>,
    pub date_end: Option<sea_orm::prelude::Date>,
    pub endorsement_billing_period_type_id: Option<i32>,
    pub retainer_fee: Option<Option<Decimal>>,
    pub remarks: Option<Option<String>>,
    pub endorsement_method: Option<Option<String>>,
    pub is_active: Option<bool>,
}

//
// ---- Handlers
//

/// GET /endorsements?page=1&page_size=20
pub async fn get_all_endorsements(
    State(state): State<AppState>,
    Query(q): Query<ListQuery>,
) -> Result<Json<PageResponse<EndorsementListRow>>, StatusCode> {
    // ---- Optional permission check (keep if you use it)
    // let has_permission = role_has_permission_by_data_object_name(
    //     &state.db,
    //     auth.claims.role_id,
    //     "endorsement",
    //     PermissionActionEnum::Read,
    // )
    // .await
    // .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // if !has_permission {
    //     return Err(StatusCode::FORBIDDEN);
    // }

    let page = q.page.unwrap_or(1).max(1);
    let page_size = q.page_size.unwrap_or(25).clamp(1, 200);

    // Join + select_only to pull related "name" columns
    let base = endorsement::Entity::find()
        .join(JoinType::LeftJoin, endorsement::Relation::Hmo.def())
        .join(
            JoinType::LeftJoin,
            endorsement::Relation::EndorsementCompany.def(),
        )
        .join(JoinType::LeftJoin, endorsement::Relation::EndorsementType.def())
        .join(
            JoinType::LeftJoin,
            endorsement::Relation::EndorsementBillingPeriodType.def(),
        )
        .select_only()
        // base columns
        .column(endorsement::Column::Id)
        .column(endorsement::Column::HmoId)
        .column(endorsement::Column::EndorsementCompanyId)
        .column(endorsement::Column::EndorsementTypeId)
        .column(endorsement::Column::AgreementCorpNumber)
        .column(endorsement::Column::DateStart)
        .column(endorsement::Column::DateEnd)
        .column(endorsement::Column::EndorsementBillingPeriodTypeId)
        .column(endorsement::Column::RetainerFee)
        .column(endorsement::Column::Remarks)
        .column(endorsement::Column::EndorsementMethod)
        .column(endorsement::Column::IsActive)
        // joined names (pick the columns you want to represent “name”)
        .column_as(hmo::Column::ShortName, "hmo_name")
        .column_as(endorsement_company::Column::Name, "company_name")
        .column_as(endorsement_type::Column::Name, "type_name")
        .column_as(
            endorsement_billing_period_type::Column::Name,
            "billing_period_type_name",
        )
        .order_by_desc(endorsement::Column::Id)
        .into_model::<EndorsementListRow>();

    let paginator = base.paginate(&state.db, page_size);

    let total = paginator
        .num_items()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let items: Vec<EndorsementListRow> = paginator
        .fetch_page(page - 1) // paginator is 0-based
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|row| row) // already EndorsementListRow
        .collect();

    Ok(Json(PageResponse {
        page,
        page_size,
        total,
        items,
    }))
}

/// GET /endorsements/:id
pub async fn get_endorsement_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<EndorsementResponse>, StatusCode> {
    let found = endorsement::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let Some(model) = found else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(model.into()))
}

/// POST /endorsements
pub async fn create_endorsement(
    State(state): State<AppState>,
    Json(body): Json<CreateEndorsementRequest>,
) -> Result<Json<EndorsementResponse>, StatusCode> {
    // (Optional) reject bad ranges early
    if body.date_end < body.date_start {
        return Err(StatusCode::BAD_REQUEST);
    }

    let active = endorsement::ActiveModel {
        hmo_id: Set(body.hmo_id),
        endorsement_company_id: Set(body.endorsement_company_id),
        endorsement_type_id: Set(body.endorsement_type_id),
        agreement_corp_number: Set(body.agreement_corp_number),
        date_start: Set(body.date_start),
        date_end: Set(body.date_end),
        endorsement_billing_period_type_id: Set(body.endorsement_billing_period_type_id),
        retainer_fee: Set(body.retainer_fee),
        remarks: Set(body.remarks),
        endorsement_method: Set(body.endorsement_method),
        is_active: Set(body.is_active),
        ..Default::default()
    };

    let inserted = active
        .insert(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(inserted.into()))
}

/// PATCH /endorsements/:id
pub async fn patch_endorsement(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(body): Json<PatchEndorsementRequest>,
) -> Result<Json<EndorsementResponse>, StatusCode> {
    let existing = endorsement::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let Some(existing) = existing else {
        return Err(StatusCode::NOT_FOUND);
    };
    let old_date_start = existing.date_start;
    let old_date_end = existing.date_end;

    let mut am: endorsement::ActiveModel = existing.into();

    if let Some(v) = body.hmo_id {
        am.hmo_id = Set(v);
    }
    if let Some(v) = body.endorsement_company_id {
        am.endorsement_company_id = Set(v);
    }
    if let Some(v) = body.endorsement_type_id {
        am.endorsement_type_id = Set(v);
    }
    if let Some(v) = body.agreement_corp_number {
        am.agreement_corp_number = Set(v);
    }
    if let Some(v) = body.date_start {
        am.date_start = Set(v);
    }
    if let Some(v) = body.date_end {
        am.date_end = Set(v);
    }
    if let Some(v) = body.endorsement_billing_period_type_id {
        am.endorsement_billing_period_type_id = Set(v);
    }
    if let Some(v) = body.retainer_fee {
        am.retainer_fee = Set(v);
    }
    if let Some(v) = body.remarks {
        am.remarks = Set(v);
    }
    if let Some(v) = body.endorsement_method {
        am.endorsement_method = Set(v);
    }
    if let Some(v)=body.is_active {
        am.is_active = Set(v);
    }

    // Optional: validate date range if either changed
    // (Pull current values out of the ActiveModel and compare.)
    let ds = am.date_start.clone().take().unwrap_or(old_date_start);
    let de = am.date_end.clone().take().unwrap_or(old_date_end);
    if de < ds {
        return Err(StatusCode::BAD_REQUEST);
    }

    let updated = am
        .update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(updated.into()))
}