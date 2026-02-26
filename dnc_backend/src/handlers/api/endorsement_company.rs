use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sea_orm::{
    ActiveModelTrait,  EntityTrait, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::AppState;
use crate::entities::endorsement_company;

// -------------------------
// DTOs
// -------------------------

#[derive(Debug, Serialize)]
pub struct EndorsementCompanyResponse {
    pub id: i32,
    pub name: String,
}

impl From<endorsement_company::Model> for EndorsementCompanyResponse {
    fn from(m: endorsement_company::Model) -> Self {
        Self { id: m.id, name: m.name }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateEndorsementCompanyRequest {
    pub name: String,
}

// -------------------------
// GET /endorsement_companies
// -------------------------
#[instrument(skip(state), err(Debug))]
pub async fn get_endorsement_companies(
    State(state): State<AppState>,
) -> Result<Json<Vec<EndorsementCompanyResponse>>, StatusCode> {
    let rows = endorsement_company::Entity::find()
        .order_by_asc(endorsement_company::Column::Name)
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to load endorsement companies: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(rows.into_iter().map(Into::into).collect()))
}

// -------------------------
// POST /endorsement_companies
// -------------------------
#[instrument(skip(state, payload), err(Debug))]
pub async fn post_endorsement_company(
    State(state): State<AppState>,
    Json(payload): Json<CreateEndorsementCompanyRequest>,
) -> Result<(StatusCode, Json<EndorsementCompanyResponse>), StatusCode> {
    let name = payload.name.trim().to_string();
    if name.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let am = endorsement_company::ActiveModel {
        name: Set(name),
        ..Default::default()
    };

    let created = am.insert(&state.db).await.map_err(|e| {
        tracing::error!("Failed to insert endorsement_company: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::CREATED, Json(created.into())))
}
