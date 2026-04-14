use chrono::{DateTime, Datelike, Days, LocalResult, TimeZone, Utc, Weekday};
use chrono_tz::Asia::Manila;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tracing::{error, info, instrument};

use dnc_backend::AppState;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::sea_query::Expr;

use crate::{entities::{verification}};

/// Starts the in-process background worker.
///
/// Behavior:
/// - runs once immediately on startup
/// - then sleeps until the next midnight in Asia/Manila
/// - repeats forever
pub fn start_daily_worker(state: AppState) -> JoinHandle<()> {
    tokio::spawn(async move {
        info!(target: "jobs", "Daily worker started");

        // Run once immediately on startup
        match run_daily_job_once(state.clone()).await {
            Ok(()) => info!(target: "jobs", "Startup daily job run completed successfully"),
            Err(err) => error!(target: "jobs", "Startup daily job run failed: {}", err),
        }

        loop {
            let now_utc = Utc::now();
            let next_run_utc = next_manila_midnight_utc(now_utc);
            let next_run_manila = next_run_utc.with_timezone(&Manila);

            let sleep_duration = match (next_run_utc - now_utc).to_std() {
                Ok(duration) => duration,
                Err(_) => std::time::Duration::from_secs(60),
            };

            info!(
                target: "jobs",
                "Daily worker sleeping until next Manila midnight: Manila={} UTC={} sleep_for={:?}",
                next_run_manila.format("%Y-%m-%d %H:%M:%S"),
                next_run_utc.format("%Y-%m-%d %H:%M:%S UTC"),
                sleep_duration
            );

            sleep(sleep_duration).await;

            match run_daily_job_once(state.clone()).await {
                Ok(()) => info!(target: "jobs", "Scheduled daily job run completed successfully"),
                Err(err) => error!(target: "jobs", "Scheduled daily job run failed: {}", err),
            }
        }
    })
}

/// Computes the next midnight in Asia/Manila, returned in UTC.
fn next_manila_midnight_utc(now_utc: DateTime<Utc>) -> DateTime<Utc> {
    let now_manila = now_utc.with_timezone(&Manila);

    let next_date = now_manila
        .date_naive()
        .checked_add_days(Days::new(1))
        .expect("date overflow while computing next Manila midnight");

    let next_midnight_manila = match Manila.with_ymd_and_hms(
        next_date.year(),
        next_date.month(),
        next_date.day(),
        0,
        0,
        0,
    ) {
        LocalResult::Single(dt) => dt,
        LocalResult::Ambiguous(dt, _) => dt,
        LocalResult::None => panic!("Could not compute next Manila midnight"),
    };

    next_midnight_manila.with_timezone(&Utc)
}


// region: run_daily_job_once()
// run_daily_job_once() runs once a day to:
#[instrument(skip(state), err)]
async fn run_daily_job_once(
    state: AppState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    // ---- 0. setup variables.
    let db = &state.db;

    let now_manila = chrono::Utc::now().with_timezone(&Manila);
    let today_manila = now_manila.date_naive();

    info!(
        target: "jobs",
        "run_daily_job_once() started at Manila={}",
        now_manila.format("%Y-%m-%d %H:%M:%S")
    );

    // ---- 1. Load only verifications that are still in status_id = 2
    let pending_verifications = verification::Entity::find()
        .filter(verification::Column::StatusId.eq(2))
        .all(db)
        .await?;

    // 2. Decide which ones are older than 7 days, excluding Sundays
    let ids_to_expire: Vec<i32> = pending_verifications
        .into_iter()
        .filter(|verification| {
            let created_date_manila = verification
                .date_created
                .with_timezone(&Manila)
                .date_naive();

            let elapsed_non_sunday_days =
                count_elapsed_days_excluding_sundays(created_date_manila, today_manila);

            elapsed_non_sunday_days > 7
        })
        .map(|verification| verification.id)
        .collect();

    if ids_to_expire.is_empty() {
        info!(
            target: "jobs",
            "run_daily_job_once() finished: no verifications needed status update"
        );
        return Ok(());
    }

    // 3. Update all matching verifications to status_id = 999
    let update_result = verification::Entity::update_many()
        .col_expr(verification::Column::StatusId, Expr::value(999))
        .filter(verification::Column::Id.is_in(ids_to_expire.clone()))
        .exec(db)
        .await?;

    info!(
        target: "jobs",
        "run_daily_job_once() finished: updated {} verification(s) to status_id=999; ids={:?}",
        update_result.rows_affected,
        ids_to_expire
    );

    Ok(())
}

// endregion: run_daily_job_once()


fn count_elapsed_days_excluding_sundays(
    start_date: chrono::NaiveDate,
    end_date: chrono::NaiveDate,
) -> i64 {
    use chrono::{Days, Weekday};

    if end_date <= start_date {
        return 0;
    }

    let mut count = 0_i64;
    let mut current = start_date
        .checked_add_days(Days::new(1))
        .expect("date overflow while counting elapsed days");

    while current < end_date {
        if current.weekday() != Weekday::Sun {
            count += 1;
        }

        current = current
            .checked_add_days(Days::new(1))
            .expect("date overflow while counting elapsed days");
    }

    count
}
