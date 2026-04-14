use chrono::{DateTime, Datelike, Days, LocalResult, TimeZone, Utc};
use chrono_tz::Asia::Manila;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tracing::{error, info, instrument};

use dnc_backend::AppState;

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

/// Executes one run of the daily job.
///
/// Put your database update logic here later.
#[instrument(skip(state), err)]
async fn run_daily_job_once(
    state: AppState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db = &state.db;

    let _ = db;

    info!(target: "jobs", "run_daily_job_once() called");

    Ok(())
}