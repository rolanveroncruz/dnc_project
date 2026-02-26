use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::AppState;
use crate::entities::endorsement_type;

// Optional query params: /endorsement-types?active_only=true
#[derive(Debug, Deserialize)]
pub struct GetEndorsementTypesQuery {
    pub active_only: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct EndorsementTypeResponse {
    pub id: i32,
    pub name: String,
    pub is_active: Option<bool>,
}

impl From<endorsement_type::Model> for EndorsementTypeResponse {
    fn from(m: endorsement_type::Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            is_active: m.is_active,
        }
    }
}

/// GET /api/endorsement-types
#[instrument(skip(state), err(Debug))]
pub async fn get_endorsement_types(
    State(state): State<AppState>,
    Query(q): Query<GetEndorsementTypesQuery>,
) -> Result<Json<Vec<EndorsementTypeResponse>>, StatusCode> {
    let mut stmt = endorsement_type::Entity::find().order_by_asc(endorsement_type::Column::Name);

    // If you prefer "NULL means active", adjust this logic.
    if q.active_only.unwrap_or(false) {
        stmt = stmt.filter(endorsement_type::Column::IsActive.eq(true));
    }

    let rows = stmt
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to load endorsement_types: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(rows.into_iter().map(Into::into).collect()))
}
