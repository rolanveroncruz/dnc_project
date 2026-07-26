use axum::body::Body;
use axum::extract::{Path as AxumPath, State};
use axum::http::{header, StatusCode};
use axum::response::Response;
use axum::Json;
use sea_orm::entity::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::entities::dentist_applications;
use crate::AppState;

// region: Get Dentist Applications
#[derive(Debug, Serialize)]
pub struct DentistApplicationListRow {
    pub id: i32,
    pub date_submitted: DateTimeWithTimeZone,

    pub name: String,
    pub clinic_name: String,
    pub contact_numbers: String,
    pub email: String,

    pub clinic_ownership_type: Option<String>,
    pub hmo_affiliations: Option<String>,
    pub clinic_address: Option<String>,

    // These are guarded download URLs returned to the frontend,
    // not raw server filesystem paths.
    pub prc_license_file_path: Option<String>,
    pub bir_2303_file_path: Option<String>,

    pub registration_doc_file_path: Option<String>,
    pub supporting_docs_file_path1: Option<String>,

    pub status: String,
}

pub async fn get_dentist_applications_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<DentistApplicationListRow>>, (StatusCode, String)> {
    let applications = dentist_applications::Entity::find()
        .order_by_desc(dentist_applications::Column::DateSubmitted)
        .all(&state.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to load dentist applications: {err}"),
            )
        })?;

    let response = applications
        .into_iter()
        .map(|application| DentistApplicationListRow {
            id: application.id,
            date_submitted: application.date_submitted,

            name: application.name,
            clinic_name: application.clinic_name,
            contact_numbers: application.contact_numbers,
            email: application.email,

            clinic_ownership_type: application.clinic_ownership_type,
            hmo_affiliations: application.hmo_affiliations,
            clinic_address: application.clinic_address,

            // Return API download URL, not the stored path.
            prc_license_file_path: non_empty_string(application.prc_license_file_path).map(|_| {
                format!(
                    "/api/website/dentist_applications/{}/documents/prc_license",
                    application.id
                )
            }),

            // DB field is bir2303_file_path, but response field remains bir_2303_file_path.
            bir_2303_file_path: non_empty_string(application.bir2303_file_path).map(|_| {
                format!(
                    "/api/website/dentist_applications/{}/documents/bir2303",
                    application.id
                )
            }),

            registration_doc_file_path: non_empty_optional_string(
                application.registration_doc_file_path
            )
                .map(|_|{
                format!(
                    "/api/website/dentist_applications/{}/documents/registration_doc",
                    application.id
                )
            }),

            supporting_docs_file_path1: non_empty_optional_string(
                application.supporting_docs_file_path1
            )
                .map(|_|{
                    format!(
                        "/api/website/dentist_applications/{}/documents/supporting_docs1",
                        application.id
                    )
                }),

            status: application.status,
        })
        .collect();

    Ok(Json(response))
}

pub async fn download_dentist_application_document_handler(
    State(state): State<AppState>,
    AxumPath((application_id, document_type)): AxumPath<(i32, String)>,
) -> Result<Response<Body>, (StatusCode, String)> {
    let application = dentist_applications::Entity::find_by_id(application_id)
        .one(&state.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to load dentist application: {err}"),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                "Dentist application not found.".to_string(),
            )
        })?;

    let stored_file_path = match document_type.as_str() {
        "prc_license" => non_empty_string(application.prc_license_file_path),
        "bir2303" => non_empty_string(application.bir2303_file_path),
        "registration_doc" => non_empty_optional_string(application.registration_doc_file_path),
        "supporting_docs1" => non_empty_optional_string(application.supporting_docs_file_path1),
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                "Invalid document type.".to_string(),
            ));
        }
    }
        .ok_or_else(||{
            (
                StatusCode::NOT_FOUND,
                "Document file path not found.".to_string(),)
        })?;



    let path = PathBuf::from(&stored_file_path);

    let file_bytes = tokio::fs::read(&path)
        .await
        .map_err(|err| {
            (
                StatusCode::NOT_FOUND,
                format!("Document file could not be read: {err}"),
            )
        })?;

    let safe_file_name = download_file_name_from_path(&path, &document_type);
    let safe_file_name = ensure_filename_has_extension(safe_file_name, &path);

    let content_disposition = build_content_disposition_header(&safe_file_name);


    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type_from_path(&path))
        .header(header::CONTENT_DISPOSITION, content_disposition)
        // Important for Angular/browser CORS:
        // lets HttpClient read the Content-Disposition header.
        .header(
            header::ACCESS_CONTROL_EXPOSE_HEADERS,
            "content-disposition, content-type",
        )
        .body(Body::from(file_bytes))
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to build download response: {err}"),
            )
        })
}

fn non_empty_string(value: String) -> Option<String> {
    let cleaned = value.trim().to_string();

    if cleaned.is_empty() {
        None
    } else {
        Some(cleaned)
    }
}

fn download_file_name_from_path(path: &PathBuf, document_type: &str) -> String {
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("dentist-application-document")
        .to_string();

    let cleaned_file_name = match document_type {
        "prc_license" => file_name
            .strip_prefix("prc_license_")
            .unwrap_or(&file_name)
            .to_string(),

        "bir2303" => file_name
            .strip_prefix("bir_2303_")
            .unwrap_or(&file_name)
            .to_string(),

        "registration_doc" => file_name
            .strip_prefix("registration_doc_")
            .unwrap_or(&file_name)
            .to_string(),

        // ✅ NEW
        "supporting_docs1" => file_name
            .strip_prefix("supporting_docs1_")
            .unwrap_or(&file_name)
            .to_string(),

        _ => file_name,
    };

    sanitize_download_filename(&cleaned_file_name)
}

fn ensure_filename_has_extension(file_name: String, path: &PathBuf) -> String {
    if file_name.contains('.') {
        return file_name;
    }

    let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
        return file_name;
    };

    if extension.trim().is_empty() {
        return file_name;
    }

    format!("{file_name}.{extension}")
}

fn sanitize_download_filename(file_name: &str) -> String {
    file_name
        .replace('"', "")
        .replace('\\', "")
        .replace('/', "")
}

fn build_content_disposition_header(file_name: &str) -> String {
    let safe_file_name = sanitize_download_filename(file_name);
    let encoded_file_name = percent_encode_header_value(&safe_file_name);

    format!(
        "attachment; filename=\"{}\"; filename*=UTF-8''{}",
        safe_file_name,
        encoded_file_name
    )
}

fn percent_encode_header_value(value: &str) -> String {
    let mut encoded = String::new();

    for byte in value.as_bytes() {
        let ch = *byte as char;

        if ch.is_ascii_alphanumeric() || ch == '.' || ch == '-' || ch == '_' {
            encoded.push(ch);
        } else {
            encoded.push_str(&format!("%{:02X}", byte));
        }
    }

    encoded
}

fn content_type_from_path(path: &PathBuf) -> &'static str {
    match path
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase())
        .as_deref()
    {
        Some("pdf") => "application/pdf",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("doc") => "application/msword",
        Some("docx") => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        Some("xls") => "application/vnd.ms-excel",
        Some("xlsx") => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        _ => "application/octet-stream",
    }
}

fn non_empty_optional_string(value: Option<String>) -> Option<String> {
    value.and_then(non_empty_string)
}
// endregion: Get Dentist Applications

// region: Patch Dentist Application Status
#[derive(Debug, Deserialize)]
pub struct UpdateDentistApplicationStatusRequest {
    pub status: String,
}



#[derive(Debug, Serialize)]
pub struct UpdateDentistApplicationStatusResponse {
    pub id: i32,
    pub status: String,
    pub message: String,
}
pub async fn update_dentist_application_status_handler(
    State(state): State<AppState>,
    AxumPath(application_id): AxumPath<i32>,
    Json(payload): Json<UpdateDentistApplicationStatusRequest>,
) -> Result<Json<UpdateDentistApplicationStatusResponse>, (StatusCode, String)> {
    let status = validate_application_status(payload.status)?;

    let application = dentist_applications::Entity::find_by_id(application_id)
        .one(&state.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to load dentist application: {err}"),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                "Dentist application not found.".to_string(),
            )
        })?;

    let mut active_model = application.into_active_model();

    active_model.status = Set(status.clone());

    let updated_application = active_model
        .update(&state.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to update dentist application status: {err}"),
            )
        })?;

    Ok(Json(UpdateDentistApplicationStatusResponse {
        id: updated_application.id,
        status: updated_application.status,
        message: "Dentist application status updated successfully.".to_string(),
    }))
}

fn validate_application_status(
    status: String,
) -> Result<String, (StatusCode, String)> {
    let normalized = status
        .trim()
        .to_ascii_lowercase()
        .replace([' ', '-'], "_");

    match normalized.as_str() {
        "new"
        | "for_evaluation"
        | "declined"
        | "accredited" => Ok(normalized),

        _ => Err((
            StatusCode::BAD_REQUEST,
            "Status must be one of: new, for_evaluation, declined, accredited."
                .to_string(),
        )),
    }
}

// endregion: Patch Dentist Application Status
