use chrono::{DateTime, Utc};
use regex::Regex;

use crate::{LineIter, LogEvent, Seconds};

pub(crate) struct AnalyzeC {
    target_name: String,
    begin_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
}

impl AnalyzeC {
    pub fn new(start_log: &str) -> Option<Self> {
        Self::get_start_target_name(start_log).map(|target_name| Self {
            target_name,
            begin_time: crate::extract_timestamp(start_log).unwrap(),
            end_time: None,
        })
    }

    #[inline(always)]
    fn start_re() -> Regex {
        Regex::new(r"Scanning dependencies of target (\w+)").unwrap()
    }

    #[inline(always)]
    fn end_re() -> Regex {
        Regex::new(r"Built target (\w+)").unwrap()
    }

    pub(crate) fn get_start_target_name(log: &str) -> Option<String> {
        let start_re = Self::start_re();
        start_re
            .captures(log)
            .map(|capture| capture[1].trim().to_owned())
    }

    pub(crate) fn target_name(&self) -> String {
        self.target_name.clone()
    }
}

#[async_trait::async_trait]
impl LogEvent for AnalyzeC {
    async fn get_duration(&mut self, mut line_iter: LineIter) -> (Option<Seconds>, LineIter) {
        let end_re = Self::end_re();
        while let Some(line) = line_iter.next_line().await.unwrap() {
            if let Some(capture) = end_re.captures(&line) {
                let target_name = capture[1].to_string();
                assert_eq!(self.target_name, target_name);
                self.end_time = crate::extract_timestamp(&line);
                return (
                    self.end_time.map(|end_time| {
                        end_time
                            .signed_duration_since(self.begin_time)
                            .num_milliseconds() as Seconds
                            / 1000.
                    }),
                    line_iter,
                );
            }
        }
        (None, line_iter)
    }
}
