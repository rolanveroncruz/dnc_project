use axum::{extract::{Multipart, Path},
           http::{header, HeaderMap, StatusCode},
           response::{IntoResponse, Response},
           Json};
use std::{path::{Path as StdPath, PathBuf}, ffi::OsStr};
use tokio::{fs, io::AsyncWriteExt};

/// POST /api/dentists/:dentist_id/contract-file
///
/// Expects multipart/form-data with a single file field (any field name).
/// Saves to: ./DNC_DATAFILES/contracts/{dentist_id}/{original_filename}
/// Returns: the full path as String (JSON or plain text - here we return plain text).
use serde::Serialize;
use chrono::Utc;

#[derive(Debug, Serialize)]
pub struct StoredContractFileResponse {
    pub file_name: String,
    pub file_path: String,
    pub content_type: String,
    pub size_bytes: i32,
    pub updated_at: String,
}

/// POST /api/dentists/:dentist_id/contract-file
///
/// Expects multipart/form-data with a single file field (any field name).
/// Saves to: ./DNC_DATAFILES/contracts/{dentist_id}/{original_filename}
/// Returns: JSON with file metadata.
pub async fn save_contract_file_for_dentist_id(
    Path(dentist_id): Path<i32>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, StatusCode> {
    let base_dir: PathBuf = ["./DNC_DATAFILES", "contracts", &dentist_id.to_string()]
        .iter()
        .collect();

    fs::create_dir_all(&base_dir)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
    {
        let file_name = field
            .file_name()
            .map(|s| s.to_string())
            .ok_or(StatusCode::BAD_REQUEST)?;

        // Basic filename safety: strip any path components
        let safe_file_name = StdPath::new(&file_name)
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or(StatusCode::BAD_REQUEST)?
            .to_string();

        let content_type = field
            .content_type()
            .map(|ct| ct.to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        let full_path = base_dir.join(&safe_file_name);

        let data = field
            .bytes()
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        // size in bytes (cap at i32::MAX to satisfy your schema)
        let size_u64 = data.len() as u64;
        let size_bytes: i32 = if size_u64 > i32::MAX as u64 {
            i32::MAX
        } else {
            size_u64 as i32
        };

        let mut f = fs::File::create(&full_path)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        f.write_all(&data)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let resp = StoredContractFileResponse {
            file_name: safe_file_name,
            file_path: full_path.to_string_lossy().to_string(),
            content_type,
            size_bytes,
            updated_at: Utc::now().to_rfc3339(),
        };

        return Ok((StatusCode::OK, Json(resp)));
    }

    Err(StatusCode::BAD_REQUEST)
}



/// GET /api/dentists/:dentist_id/contract-file/:file_name
///
/// Serves: ./DNC_DATAFILES/contracts/{dentist_id}/{file_name}
pub async fn get_contract_file_for_dentist_id(
    Path((dentist_id, file_name)): Path<(i32, String)>,
) -> Result<Response, StatusCode> {
    // Basic filename safety: strip any path components
    let safe_file_name = StdPath::new(&file_name)
        .file_name()
        .and_then(OsStr::to_str)
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_string();

    let full_path: PathBuf = ["./DNC_DATAFILES", "contracts", &dentist_id.to_string()]
        .iter()
        .collect::<PathBuf>()
        .join(&safe_file_name);

    let bytes = fs::read(&full_path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    // Minimal content-type handling (default octet-stream)
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/octet-stream"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        header::HeaderValue::from_str(&format!("inline; filename=\"{}\"", safe_file_name))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );

    Ok((StatusCode::OK, headers, bytes).into_response())
}
