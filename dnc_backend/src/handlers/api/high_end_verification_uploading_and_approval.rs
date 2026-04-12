use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::{
        header::{CONTENT_DISPOSITION, CONTENT_TYPE},
        HeaderValue, StatusCode,
    },
    response::{Response},
    Json,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use std::path::{Path as StdPath, PathBuf};
use chrono::Utc;
use tokio::fs;

use uuid::Uuid;
use crate::AppState;
use crate::entities::{high_end_files, verification};

use serde::Serialize;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Serialize)]
pub struct UploadedHighEndFileResponse {
    pub id: i32,
    pub verification_id: i32,
    pub filename: String,
    pub original_filename: String,
}

#[derive(Debug, Serialize)]
pub struct HighEndFileListItem {
    pub id: i32,
    pub verification_id: i32,
    pub filename: String,
}

// region: Helper Functions
fn high_end_files_dir() -> PathBuf {
    PathBuf::from("./uploads/high_end_files")
}


// sanitize_filename() takes the original filename and replaces some characters with '_'.
// This is meant to prevent filesystem issues.
fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

// make_stored_filename takes the sanitized original filename and prepends it with a timestamp, and the uuid.
fn make_stored_filename(original_filename: &str) -> String {
    let sanitized = sanitize_filename(original_filename);
    let timestamp = Utc::now().timestamp();
    let uuid = Uuid::new_v4();
    format!("{}_{}_{}", timestamp, uuid, sanitized)
}

async fn ensure_high_end_files_dir_exists() -> Result<(), std::io::Error> {
    fs::create_dir_all(high_end_files_dir()).await
}

fn guess_content_type(filename: &str) -> &'static str {
    match StdPath::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_ascii_lowercase())
        .as_deref()
    {
        Some("pdf") => "application/pdf",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("doc") => "application/msword",
        Some("docx") => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        Some("xls") => "application/vnd.ms-excel",
        Some("xlsx") => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        Some("txt") => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}
fn internal_error<E: std::fmt::Display>(err: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

// endregion: Helper Functions



// region: upload_high_end_file()
pub async fn upload_high_end_file(
    State(state): State<AppState>,
    Path(verification_id): Path<i32>,
    mut multipart: Multipart,
) -> Result<Json<UploadedHighEndFileResponse>, (StatusCode, String)> {
    let db: &DatabaseConnection = &state.db;

    // ---- 1. confirm that the verification_id leads to a legit verification record.
    // If it doesn't exist, return 404.
    let verification_exists = verification::Entity::find_by_id(verification_id)
        .one(db)
        .await
        .map_err(internal_error)?
        .is_some();

    if !verification_exists {
        return Err((StatusCode::NOT_FOUND, "Verification not found".to_string()));
    }

    // ---- 2. Before writing, ensure the high_end_files_dir (./uploads/high_end_files/ exists,
    // else return 500
    ensure_high_end_files_dir_exists()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // ---- 3. declare variables to collect multipart data.
    let mut uploaded_file_bytes: Option<bytes::Bytes> = None;
    let mut uploaded_original_filename: Option<String> = None;
    let mut description: Option<String> = None;


    // ---- 4. Loop through all multipart fields.
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
    {
        // ---- 4a. Capture field metadata before consuming the field.
        let field_name = field.name().map(|s| s.to_string());
        let file_name = field.file_name().map(|s| s.to_string());

        // ---- 4b. If this part has a filename, treat it as the uploaded file.
        if let Some(original_filename) = file_name {
            let bytes = field
                .bytes()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

            uploaded_original_filename = Some(original_filename);
            uploaded_file_bytes = Some(bytes);
            continue;
        }
        // ---- 4c. Otherwise, if this is the "description" field, read it as text.
        if field_name.as_deref() == Some("description") {
            let text = field
                .text()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
            let trimmed = text.trim().to_string();
            description = if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            };
        }
    } // end of while


    //---- 5. Require that a file was provided.
    let original_filename = uploaded_original_filename
        .ok_or((StatusCode::BAD_REQUEST, "Uploaded file has no filename".to_string()))?;

    let bytes = uploaded_file_bytes
        .ok_or((StatusCode::BAD_REQUEST, "No file uploaded".to_string()))?;

    // ---- 6. Create a unique and sanitized filename and path from the original filename.
    let stored_filename = make_stored_filename(&original_filename);
    let full_path = high_end_files_dir().join(&stored_filename);

    // 7. Create the file and save the bytes.
    let mut file = fs::File::create(&full_path)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    file.write_all(&bytes)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;


    // ---- 8. Prepare model for insertion.
    let active_model = high_end_files::ActiveModel {
        verification_id: Set(verification_id),
        filename: Set(stored_filename.clone()),
        original_filename: Set(Some(original_filename.clone())),
        // ✅ 9. Save description too.
        description: Set(description),
        ..Default::default()
    };

    // ---- 9. Insert into db.
    let inserted = active_model
        .insert(db)
        .await
        .map_err(internal_error)?;

    // ---- 10. Update the verification model.
    let verification_model = verification::Entity::find_by_id(verification_id)
        .one(db)
        .await
        .map_err(internal_error)?
        .ok_or((StatusCode::NOT_FOUND, "Verification not found".to_string()))?;

    let mut verification_active_model : verification::ActiveModel = verification_model.into();
    verification_active_model.status_id= Set(21);

    verification_active_model
        .update(db)
        .await
        .map_err(internal_error)?;

    // ---- 11. Return the result.
    Ok(Json(UploadedHighEndFileResponse {
        id: inserted.id,
        verification_id: inserted.verification_id,
        filename: inserted.filename,
        original_filename,
    }))
}
// endregion: upload_high_end_file()



// region: list_uploaded_high_end_files()
pub async fn list_uploaded_high_end_files(
    State(state): State<AppState>,
    Path(verification_id): Path<i32>,
) -> Result<Json<Vec<HighEndFileListItem>>, (StatusCode, String)> {
    let db: &DatabaseConnection = &state.db;

    let verification_exists = verification::Entity::find_by_id(verification_id)
        .one(db)
        .await
        .map_err(internal_error)?
        .is_some();

    if !verification_exists {
        return Err((StatusCode::NOT_FOUND, "Verification not found".to_string()));
    }

    let files = high_end_files::Entity::find()
        .filter(high_end_files::Column::VerificationId.eq(verification_id))
        .all(db)
        .await
        .map_err(internal_error)?;

    let response = files
        .into_iter()
        .map(|f| HighEndFileListItem {
            id: f.id,
            verification_id: f.verification_id,
            filename: f.filename,
        })
        .collect();

    Ok(Json(response))
}

// endregion: list_uploaded_high_end_files()



// region: download_high_end_file()
pub async fn download_high_end_file(
    State(state): State<AppState>,
    Path(high_end_file_id): Path<i32>,
) -> Result<Response, (StatusCode, String)> {
    let db: &DatabaseConnection = &state.db;

    let file_row = high_end_files::Entity::find_by_id(high_end_file_id)
        .one(db)
        .await
        .map_err(internal_error)?
        .ok_or((StatusCode::NOT_FOUND, "File not found".to_string()))?;

    let full_path = high_end_files_dir().join(&file_row.filename);

    let bytes = fs::read(&full_path)
        .await
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                (StatusCode::NOT_FOUND, "Physical file not found on disk".to_string())
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        })?;

    let content_type = guess_content_type(&file_row.filename);

    let mut response = Response::new(Body::from(bytes));
    *response.status_mut() = StatusCode::OK;

    response.headers_mut().insert(
        CONTENT_TYPE,
        HeaderValue::from_str(content_type)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
    );

    response.headers_mut().insert(
        CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{}\"", file_row.filename))
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
    );

    Ok(response)
}

// endregion: download_high_end_file()