use anyhow::{anyhow, bail, Result};
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime};
use rust_decimal::Decimal;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde_json::Value as JsonValue;
use std::str::FromStr;

use crate::entities::app_config;

/// ✅ Internal helper: fetches one config row by key and verifies the expected value_type.
async fn get_app_config_value(
    db: &DatabaseConnection,
    key: &str,
    expected_type: &str,
) -> Result<String> {
    let row = app_config::Entity::find()
        .filter(app_config::Column::Key.eq(key))
        .one(db)
        .await?
        .ok_or_else(|| anyhow!("App config key not found: {}", key))?;

    if row.value_type != expected_type {
        bail!(
            "App config key '{}' has value_type '{}', expected '{}'",
            key,
            row.value_type,
            expected_type
        );
    }

    Ok(row.value)
}

/// ✅ Gets a string config value.
pub async fn get_app_config_string(
    db: &DatabaseConnection,
    key: &str,
) -> Result<String> {
    get_app_config_value(db, key, "string").await
}

/// ✅ Gets an integer config value.
pub async fn get_app_config_integer(
    db: &DatabaseConnection,
    key: &str,
) -> Result<i32> {
    let value = get_app_config_value(db, key, "integer").await?;

    value
        .parse::<i32>()
        .map_err(|err| anyhow!("Invalid integer value for app config '{}': {}", key, err))
}

/// ✅ Gets a decimal config value.
pub async fn get_app_config_decimal(
    db: &DatabaseConnection,
    key: &str,
) -> Result<Decimal> {
    let value = get_app_config_value(db, key, "decimal").await?;

    Decimal::from_str(&value)
        .map_err(|err| anyhow!("Invalid decimal value for app config '{}': {}", key, err))
}

/// ✅ Gets a boolean config value.
pub async fn get_app_config_boolean(
    db: &DatabaseConnection,
    key: &str,
) -> Result<bool> {
    let value = get_app_config_value(db, key, "boolean").await?;

    value
        .parse::<bool>()
        .map_err(|err| anyhow!("Invalid boolean value for app config '{}': {}", key, err))
}

/// ✅ Gets a date config value.
///
/// Expected format:
/// "YYYY-MM-DD"
pub async fn get_app_config_date(
    db: &DatabaseConnection,
    key: &str,
) -> Result<NaiveDate> {
    let value = get_app_config_value(db, key, "date").await?;

    NaiveDate::parse_from_str(&value, "%Y-%m-%d")
        .map_err(|err| anyhow!("Invalid date value for app config '{}': {}", key, err))
}

/// ✅ Gets a datetime config value.
///
/// Recommended stored format:
/// "2026-05-07T13:45:00+08:00"
pub async fn get_app_config_datetime(
    db: &DatabaseConnection,
    key: &str,
) -> Result<DateTime<FixedOffset>> {
    let value = get_app_config_value(db, key, "datetime").await?;

    DateTime::parse_from_rfc3339(&value)
        .map_err(|err| anyhow!("Invalid datetime value for app config '{}': {}", key, err))
}

/// ✅ Gets a time config value.
///
/// Expected format:
/// "HH:MM:SS"
pub async fn get_app_config_time(
    db: &DatabaseConnection,
    key: &str,
) -> Result<NaiveTime> {
    let value = get_app_config_value(db, key, "time").await?;

    NaiveTime::parse_from_str(&value, "%H:%M:%S")
        .map_err(|err| anyhow!("Invalid time value for app config '{}': {}", key, err))
}

/// ✅ Gets a JSON config value.
pub async fn get_app_config_json(
    db: &DatabaseConnection,
    key: &str,
) -> Result<JsonValue> {
    let value = get_app_config_value(db, key, "json").await?;

    serde_json::from_str::<JsonValue>(&value)
        .map_err(|err| anyhow!("Invalid JSON value for app config '{}': {}", key, err))
}