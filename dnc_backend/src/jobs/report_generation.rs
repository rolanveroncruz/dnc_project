use anyhow::Result;
use chrono::Utc;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tracing::{error, info};

use crate::AppState;

/*
F is a function that takes an AppState and returns a Fut.
Fut is an async operation that eventually returns an anyhow::Result<()>
Send - can be safely moved across threads.
Sync - can be safely referenced across threads.
Copy - can be safely copied across threads.
'static - does not borrow temporary local data. Safe to live as long as the spawned task.
*/
pub fn start_monthly_report_worker<F, Fut>(
    state: AppState,
    report_name: &'static str,
    run_once: F,
) -> JoinHandle<()>
where
    F: Fn(AppState) -> Fut + Send + Sync + Copy + 'static,
    Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
{
    // every thing inside {} after async move is a new tokio task.
    tokio::spawn(async move {
        info!(target: "jobs", "{} monthly worker started", report_name);

        // Run once on startup.
        match run_once(state.clone()).await {
            Ok(()) => info!(target: "jobs", "{} startup check completed", report_name),
            Err(err) => error!(target: "jobs", "{} startup check failed: {}", report_name, err),
        }

        loop {
            // Set the time now as now_utc.
            let now_utc = Utc::now();

            // Determine the next run, given the report_name, and the now_utc.
            let next_run_utc = match next_monthly_report_run_utc(state.clone(), report_name, now_utc).await {
                Ok(dt) => dt,
                Err(err) => {
                    error!(
                        target: "jobs",
                        "{} failed to calculate next monthly run: {}",
                        report_name,
                        err
                    );

                    sleep(std::time::Duration::from_secs(60 * 60)).await;
                    continue;
                }
            };

            let sleep_duration = (next_run_utc - now_utc)
                .to_std()
                .unwrap_or(std::time::Duration::from_secs(60));

            info!(
                target: "jobs",
                "{} sleeping until UTC={} sleep_for={:?}",
                report_name,
                next_run_utc.format("%Y-%m-%d %H:%M:%S UTC"),
                sleep_duration
            );

            sleep(sleep_duration).await;

            match run_once(state.clone()).await {
                Ok(()) => info!(target: "jobs", "{} scheduled check completed", report_name),
                Err(err) => error!(target: "jobs", "{} scheduled check failed: {}", report_name, err),
            }
        }
    })
}
async fn next_monthly_report_run_utc(state: AppState, report_name: &str, now_utc: chrono::DateTime<Utc>) -> Result<chrono::DateTime<Utc>> {
    Ok(Utc::now())
}