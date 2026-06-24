use axum::extract::{Multipart, State};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{ActiveModelTrait, Set};
use serde::Serialize;
use std::path::{Path, PathBuf};
use tokio::fs;
use uuid::Uuid;

use crate::AppState;
use crate::entities::dentist_applications;

#[derive(Debug, Serialize)]
pub struct SubmitDentistApplicationResponse {
    pub id: i32,
    pub message: String,
}

#[derive(Debug, Default)]
struct DentistApplicationForm {
    name: Option<String>,
    clinic_name: Option<String>,
    contact_numbers: Option<String>,
    email: Option<String>,
    prc_license_file_path: Option<String>,
    bir_2303_file_path: Option<String>,
}

pub async fn submit_dentist_application_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<SubmitDentistApplicationResponse>), (StatusCode, String)> {
    let application_uuid = Uuid::new_v4().to_string();

    let upload_dir = PathBuf::from("uploads")
        .join("dentist_applications")
        .join(&application_uuid);

    fs::create_dir_all(&upload_dir)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create upload directory: {err}"),
            )
        })?;

    let mut form = DentistApplicationForm::default();

    while let Some(field) = multipart.next_field().await.map_err(|err| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid multipart form data: {err}"),
        )
    })? {
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "name" => {
                form.name = Some(read_text_field(field).await?);
            }
            "clinic_name" => {
                form.clinic_name = Some(read_text_field(field).await?);
            }
            "contact_numbers" => {
                form.contact_numbers = Some(read_text_field(field).await?);
            }
            "email" => {
                form.email = Some(read_text_field(field).await?);
            }
            "prc_license_file" => {
                let saved_path = save_uploaded_file(field, &upload_dir, "prc_license").await?;
                form.prc_license_file_path = Some(saved_path);
            }
            "bir_2303_file" => {
                let saved_path = save_uploaded_file(field, &upload_dir, "bir_2303").await?;
                form.bir_2303_file_path = Some(saved_path);
            }
            _ => {
                // Ignore unexpected fields for now.
            }
        }
    }

    let name = required(form.name, "name")?;
    let clinic_name = required(form.clinic_name, "clinic_name")?;
    let contact_numbers = required(form.contact_numbers, "contact_numbers")?;
    let email = required(form.email, "email")?;
    let prc_license_file_path = required(form.prc_license_file_path, "prc_license_file")?;
    let bir_2303_file_path = required(form.bir_2303_file_path, "bir_2303_file")?;

    let active_model = dentist_applications::ActiveModel {
        name: Set(name),
        clinic_name: Set(clinic_name),
        contact_numbers: Set(contact_numbers),
        email: Set(email),
        prc_license_file_path: Set(prc_license_file_path),
        bir2303_file_path: Set(bir_2303_file_path),
        status: Set("new".to_string()),
        ..Default::default()
    };

    let inserted = active_model
        .insert(&state.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to save dentist application: {err}"),
            )
        })?;

    Ok((
        StatusCode::CREATED,
        Json(SubmitDentistApplicationResponse {
            id: inserted.id,
            message: "Application submitted successfully.".to_string(),
        }),
    ))
}

async fn read_text_field(field: axum::extract::multipart::Field<'_>) -> Result<String, (StatusCode, String)> {
    let value = field.text().await.map_err(|err| {
        (
            StatusCode::BAD_REQUEST,
            format!("Failed to read text field: {err}"),
        )
    })?;

    Ok(value.trim().to_string())
}

async fn save_uploaded_file(
    field: axum::extract::multipart::Field<'_>,
    upload_dir: &Path,
    prefix: &str,
) -> Result<String, (StatusCode, String)> {
    let original_filename = field
        .file_name()
        .map(|value| value.to_string())
        .unwrap_or_else(|| "uploaded_file".to_string());

    let safe_filename = sanitize_filename(&original_filename);

    let final_filename = format!("{prefix}_{safe_filename}");
    let final_path = upload_dir.join(final_filename);

    let bytes = field.bytes().await.map_err(|err| {
        (
            StatusCode::BAD_REQUEST,
            format!("Failed to read uploaded file: {err}"),
        )
    })?;

    if bytes.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("{prefix} file is empty"),
        ));
    }

    // Basic 10 MB limit per file.
    const MAX_FILE_SIZE: usize = 10 * 1024 * 1024;

    if bytes.len() > MAX_FILE_SIZE {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("{prefix} file exceeds the 10 MB limit"),
        ));
    }

    fs::write(&final_path, bytes).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to save uploaded file: {err}"),
        )
    })?;

    Ok(final_path.to_string_lossy().to_string())
}

fn required(value: Option<String>, field_name: &str) -> Result<String, (StatusCode, String)> {
    match value {
        Some(value) if !value.trim().is_empty() => Ok(value),
        _ => Err((
            StatusCode::BAD_REQUEST,
            format!("{field_name} is required"),
        )),
    }
}

fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '.' || ch == '-' || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}