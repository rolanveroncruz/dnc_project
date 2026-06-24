use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sea_orm::entity::prelude::DateTimeWithTimeZone;
use sea_orm::{EntityTrait, QueryOrder};
use serde::Serialize;

use crate::entities::contact_us_messages;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ContactUsMessageListRow {
    pub id: i32,
    pub date_submitted: DateTimeWithTimeZone,

    pub person_type: String,
    pub name: String,
    pub card_number: Option<String>,
    pub company_and_hmo: Option<String>,
    pub contact_numbers: String,
    pub message: String,

    pub status: String,
}

pub async fn get_contact_us_messages_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<ContactUsMessageListRow>>, (StatusCode, String)> {
    let messages = contact_us_messages::Entity::find()
        .order_by_desc(contact_us_messages::Column::DateSubmitted)
        .all(&state.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to load contact us messages: {err}"),
            )
        })?;

    let response = messages
        .into_iter()
        .map(|message| ContactUsMessageListRow {
            id: message.id,
            date_submitted: message.date_submitted,

            person_type: message.person_type,
            name: message.name,
            card_number: message.card_number,
            company_and_hmo: message.company_and_hmo,
            contact_numbers: message.contact_numbers,
            message: message.message,

            status: message.status,
        })
        .collect();

    Ok(Json(response))
}