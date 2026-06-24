use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{DatabaseConnection, DbBackend, FromQueryResult, Statement};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct PublicDentistSearchQuery {
    pub name: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, FromQueryResult)]
pub struct PublicDentistSearchResult {
    pub dentist_id: i32,
    pub dentist_name: String,

    pub clinic_id: i32,
    pub clinic_name: String,
    pub clinic_address: String,

    pub city_name: String,
    pub region_name: String,
    pub zip_code: Option<String>,

    pub contact_numbers: Option<String>,
    pub schedule: Option<String>,

    pub special_services: Vec<String>,
}

pub async fn search_public_dentists_handler(
    State(state): State<AppState>,
    Query(query): Query<PublicDentistSearchQuery>,
) -> Result<Json<Vec<PublicDentistSearchResult>>, (StatusCode, String)> {
    let name_query = normalize_optional_query(query.name);
    let location_query = normalize_optional_query(query.location);

    if name_query.is_none() && location_query.is_none() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Please provide a dentist/clinic name or a location search value.".to_string(),
        ));
    }

    let name_pattern = name_query.map(|value| format!("%{value}%"));
    let location_pattern = location_query.map(|value| format!("%{value}%"));

    let results = search_public_dentists(
        &state.db,
        name_pattern,
        location_pattern,
    )
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to search dentists: {err}"),
            )
        })?;

    Ok(Json(results))
}

async fn search_public_dentists(
    db: &DatabaseConnection,
    name_pattern: Option<String>,
    location_pattern: Option<String>,
) -> Result<Vec<PublicDentistSearchResult>, sea_orm::DbErr> {
    let sql = r#"
        SELECT
            d.id AS dentist_id,

            trim(concat_ws(' ', d.given_name, d.middle_name, d.last_name)) AS dentist_name,

            c.id AS clinic_id,
            c.name AS clinic_name,
            c.address AS clinic_address,

            COALESCE(ci.name, '') AS city_name,
            COALESCE(r.name, '') AS region_name,
            c.zip_code AS zip_code,

            -- If dental_clinic has a contact number column, replace this NULL with it.
            -- Example: c.contact_numbers AS contact_numbers
            c.contact_numbers AS contact_numbers,

            dc.schedule AS schedule,

            COALESCE(
                array_agg(DISTINCT cc.name) FILTER (WHERE cc.name IS NOT NULL),
                ARRAY[]::text[]
            ) AS special_services

        FROM dentist_clinic dc

        INNER JOIN dentist d
            ON d.id = dc.dentist_id

        INNER JOIN dental_clinic c
            ON c.id = dc.clinic_id

        LEFT JOIN city ci
            ON ci.id = c.city_id

        LEFT JOIN province p
            ON p.id = ci.province_id

        LEFT JOIN region r
            ON r.id = p.region_id

        LEFT JOIN clinic_capabilities_list ccl
            ON ccl.clinic_id = c.id

        LEFT JOIN clinic_capability cc
            ON cc.id = ccl.capability_id
            AND cc.active = true

        WHERE dc.clinic_id IS NOT NULL

          AND (
                $1::text IS NULL
                OR trim(concat_ws(' ', d.given_name, d.middle_name, d.last_name)) ILIKE $1
                OR trim(concat_ws(' ', d.last_name, d.given_name, d.middle_name)) ILIKE $1
                OR c.name ILIKE $1
          )

          AND (
                $2::text IS NULL
                OR c.address ILIKE $2
                OR c.zip_code ILIKE $2
                OR ci.name ILIKE $2
                OR p.name ILIKE $2
                OR r.name ILIKE $2
          )

        GROUP BY
            d.id,
            d.given_name,
            d.middle_name,
            d.last_name,
            c.id,
            c.name,
            c.address,
            c.zip_code,
            ci.name,
            r.name,
            dc.schedule

        ORDER BY
            d.last_name ASC,
            d.given_name ASC,
            c.name ASC

        LIMIT 100
    "#;

    let statement = Statement::from_sql_and_values(
        DbBackend::Postgres,
        sql,
        vec![
            name_pattern.into(),
            location_pattern.into(),
        ],
    );

    PublicDentistSearchResult::find_by_statement(statement)
        .all(db)
        .await
}

fn normalize_optional_query(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}