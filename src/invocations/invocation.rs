use color_eyre::Result;
use csv::WriterBuilder;
use serde::{Deserialize, Serialize};
use std::{
    fs::OpenOptions,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use time::{OffsetDateTime, format_description};

use crate::invocations::invocation_list::InvocationList;
use crate::ui::retsyn_app::PROJECT_DIRS;

/// The minimum number of invocations to load for frecency calculations
pub(crate) const MIN_INVOCATION_NUM: usize = 1000;
/// The number of days/files that will be retained for invocation logs
pub(crate) const MAX_INVOCATION_RETENTION: usize = 60;
// TODO need to implement invocation log cleaning

/// The prefix to use for invocation log files
pub(crate) const INVOCATION_FILE_PREFIX: &str = "retsyn-invocations-";
/// The suffix to use for invocation log files
const INVOCATION_FILE_SUFFIX: &str = ".csv";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum Action {
    Open,
    OpenLink,
    Reveal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Invocation {
    pub(crate) timestamp: u64,
    pub(crate) action: Action,
    pub(crate) query: String,
    // TODO should we add the search query?
    pub(crate) path: String,
    pub(crate) title: String,
    pub(crate) url: String,
}

impl Invocation {
    pub(crate) fn new(
        action: Action,
        query: String,
        path: String,
        title: String,
        url: String,
    ) -> Self {
        // Get the current system time.
        let now = SystemTime::now();

        // Calculate the duration since Unix Epoch.
        let timestamp = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Self {
            timestamp,
            action,
            query,
            path,
            title,
            url,
        }
    }

    pub(crate) fn cache_file(date: OffsetDateTime) -> PathBuf {
        // Format date as YYYY-MM-DD
        let format = format_description::parse("[year]-[month]-[day]")
            .expect("cache file timestamp format to be vaild");
        let date_str = date
            .format(&format)
            .expect("formatting cache file timestamp to work")
            .to_string();
        let log_filename = format!(
            "{INVOCATION_FILE_PREFIX}{}{INVOCATION_FILE_SUFFIX}",
            date_str
        );
        PROJECT_DIRS.cache_dir().join(log_filename)
        // TODO need to only return valid files
    }

    pub(crate) fn append_invocations_to_csv(
        invocations: &InvocationList,
        file_path: &Path,
    ) -> Result<()> {
        // Check if the file already exists so we don't write headers more than once
        let write_headers = !file_path.exists();

        // Open the file in append mode.
        // If the file does not exist, create it.
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)?;

        // Create a CSV writer that writes to the file.
        // Note: When appending, you might want to avoid writing headers if they already exist.
        let mut wtr = WriterBuilder::new()
            .has_headers(write_headers)
            .from_writer(file);

        // Write each record in append mode
        for invocation in invocations {
            wtr.serialize(invocation)?;
        }

        // Make sure all data is flushed to the file
        wtr.flush()?;
        Ok(())
    }
}
