use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder};
use serde::Serialize;

use crate::AppState;
use crate::entities::endorsement_billing_period_type;
#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct EndorsementBillingPeriodTypeResponse {
    pub id: i32,
    pub name: String,
    pub is_active: Option<bool>,
}

pub async fn get_endorsement_billing_period_types(
    State(state): State<AppState>,
) -> Result<Json<Vec<EndorsementBillingPeriodTypeResponse>>, StatusCode> {
    let is_active_col = endorsement_billing_period_type::Column::IsActive;

    // NULL treated as active
    let active_condition = Condition::any()
        .add(is_active_col.eq(true))
        .add(is_active_col.is_null());

    let rows = endorsement_billing_period_type::Entity::find()
        .filter(active_condition)
        .order_by_asc(endorsement_billing_period_type::Column::Name)
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to load endorsement_billing_period_type list: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(
        rows.into_iter()
            .map(|m| EndorsementBillingPeriodTypeResponse {
                id: m.id,
                name: m.name,
                is_active: m.is_active,
            })
            .collect(),
    ))
}
