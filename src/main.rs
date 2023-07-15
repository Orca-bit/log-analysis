use chrono::prelude::*;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader, Lines};

mod analyze_c;
mod analyze_java;
mod event;

type Seconds = f64;
type LineIter = Lines<BufReader<File>>;

#[async_trait::async_trait]
pub(crate) trait LogEvent {
    async fn get_duration(&mut self, line_iter: LineIter) -> (Option<Seconds>, LineIter);
}

#[tokio::main]
async fn main() {
    let log_file_path = r"F:\rust_prac\log_analysis\test.log";

    if let Ok(file) = File::open(log_file_path).await {
        let reader = BufReader::new(file);
        let analyze_res = event::analyze(reader.lines()).await;
        analyze_res.iter().for_each(|s| println!("{}", s));
    }
}

fn extract_timestamp(log_entry: &str) -> Option<DateTime<Utc>> {
    let timestamp_start = log_entry.find('[')? + 1; // Find the '[' character
    let timestamp_end = log_entry.find(']')?;
    let timestamp_str = &log_entry[timestamp_start..timestamp_end];

    Utc.datetime_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S")
        .ok()
}

#[test]
fn test_extract_timestamp() {
    let time = "[2022-07-13 22:23:44]";
    assert!(extract_timestamp(time).is_some());
}
