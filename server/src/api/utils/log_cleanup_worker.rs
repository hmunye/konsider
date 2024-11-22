use chrono::TimeZone;

#[tracing::instrument(name = "log cleanup running", skip(log_dir, retention_days))]
pub async fn log_cleanup_task(log_dir: std::path::PathBuf, retention_days: u16) {
    let interval = tokio::time::Duration::from_secs(86400); // 24 hours

    loop {
        let current_time = chrono::Utc::now();

        if let Ok(entries) = std::fs::read_dir(&log_dir) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();

                if path.is_file() {
                    if let Some(file_name) = path.file_name() {
                        let file_name = file_name.to_str().unwrap_or("");

                        if let Some(timestamp_str) = file_name.strip_prefix("k6r.") {
                            let formatted_timestamp =
                                format!("{}:00:00", timestamp_str.replace("-", " "));

                            match chrono::NaiveDateTime::parse_from_str(
                                formatted_timestamp.as_str(),
                                "%Y %m %d %H:%M:%S",
                            ) {
                                Ok(timestamp) => {
                                    let file_time = chrono::Utc.from_utc_datetime(&timestamp);

                                    let age_in_days =
                                        current_time.signed_duration_since(file_time).num_days();

                                    if age_in_days > retention_days as i64 {
                                        tracing::info!("deleting old log file: {:?}", path);
                                        let _ = std::fs::remove_file(path);
                                    }
                                }
                                Err(err) => {
                                    tracing::error!("failed to parse timestamp for: {}", err);
                                }
                            }
                        }
                    }
                }
            }
        }

        tokio::time::sleep(interval).await;
    }
}
