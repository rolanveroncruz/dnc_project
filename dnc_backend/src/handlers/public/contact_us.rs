use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};

use crate::entities::contact_us_messages;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct SubmitContactUsMessageRequest {
    pub person_type: String,
    pub name: String,
    pub card_number: Option<String>,
    pub company_and_hmo: Option<String>,
    pub contact_numbers: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SubmitContactUsMessageResponse {
    pub id: i32,
    pub message: String,
}

pub async fn submit_contact_us_message_handler(
    State(state): State<AppState>,
    Json(request): Json<SubmitContactUsMessageRequest>,
) -> Result<(StatusCode, Json<SubmitContactUsMessageResponse>), (StatusCode, String)> {
    let person_type = clean_required(request.person_type, "person_type")?;
    let name = clean_required(request.name, "name")?;
    let contact_numbers = clean_required(request.contact_numbers, "contact_numbers")?;
    let message = clean_required(request.message, "message")?;

    validate_person_type(&person_type)?;

    let card_number = clean_optional(request.card_number);
    let company_and_hmo = clean_optional(request.company_and_hmo);

    let active_model = contact_us_messages::ActiveModel {
        person_type: Set(person_type),
        name: Set(name),
        card_number: Set(card_number),
        company_and_hmo: Set(company_and_hmo),
        contact_numbers: Set(contact_numbers),
        message: Set(message),
        status: Set("new".to_string()),
        ..Default::default()
    };

    let inserted = active_model
        .insert(&state.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to save contact us message: {err}"),
            )
        })?;

    Ok((
        StatusCode::CREATED,
        Json(SubmitContactUsMessageResponse {
            id: inserted.id,
            message: "Contact message submitted successfully.".to_string(),
        }),
    ))
}

fn clean_required(value: String, field_name: &str) -> Result<String, (StatusCode, String)> {
    let cleaned = value.trim().to_string();

    if cleaned.is_empty() {
        Err((
            StatusCode::BAD_REQUEST,
            format!("{field_name} is required"),
        ))
    } else {
        Ok(cleaned)
    }
}

fn clean_optional(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn validate_person_type(person_type: &str) -> Result<(), (StatusCode, String)> {
    match person_type {
        "member" | "dentist" | "broker" | "hmo_rep" => Ok(()),
        _ => Err((
            StatusCode::BAD_REQUEST,
            "person_type must be one of: member, dentist, broker, hmo_rep".to_string(),
        )),
    }
}