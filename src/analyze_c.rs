use chrono::{DateTime, Utc};
use regex::Regex;

use crate::{LineIter, LogEvent, Seconds};

pub(crate) struct AnalyzeC {
    target_name: String,
    begin_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
}

impl AnalyzeC {
    pub fn new(start_log: &str) -> Option<Self> {
        if Self::start_re().is_match(start_log) {
            let target_name = Self::get_start_target_name(start_log).unwrap();
            Some(Self {
                target_name,
                begin_time: crate::extract_timestamp(start_log).unwrap(),
                end_time: DateTime::default(),
            })
        } else {
            None
        }
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
        if start_re.is_match(log) {
            Some(start_re.captures(log).unwrap()[1].to_string())
        } else {
            None
        }
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
            if end_re.is_match(&line) {
                let target_name = end_re.captures(&line).unwrap()[1].to_string();
                assert_eq!(self.target_name, target_name);
                self.end_time = crate::extract_timestamp(&line).unwrap();
                return (
                    Some(
                        self.end_time
                            .signed_duration_since(self.begin_time)
                            .num_milliseconds() as Seconds / 1000.
                    ),
                    line_iter,
                );
            }
        }
        (None, line_iter)
    }
}
