use regex::Regex;

use crate::{LineIter, LogEvent, Seconds};

pub(crate) struct AnalyzeJava {
    project_name: String,
}

impl AnalyzeJava {
    pub fn new(start_log: &str) -> Option<Self> {
        Self::get_project_name(start_log).map(|project_name| Self { project_name })
    }

    #[inline(always)]
    fn start_re() -> Regex {
        Regex::new(r"----------------------< (.*) >----------------------").unwrap()
    }

    #[inline(always)]
    fn end_re() -> Regex {
        Regex::new(r"Total time: (.*) s").unwrap()
    }

    pub(crate) fn get_project_name(log: &str) -> Option<String> {
        let start_re = Self::start_re();
        start_re
            .captures(log)
            .map(|capture| capture[1].trim().to_owned())
    }

    pub(crate) fn project_name(&self) -> String {
        self.project_name.clone()
    }
}

#[async_trait::async_trait]
impl LogEvent for AnalyzeJava {
    async fn get_duration(&mut self, mut line_iter: LineIter) -> (Option<Seconds>, LineIter) {
        let end_re = Self::end_re();
        while let Some(line) = line_iter.next_line().await.unwrap() {
            if let Some(capture) = end_re.captures(&line) {
                return (capture[1].trim().parse::<Seconds>().ok(), line_iter);
            }
        }
        (None, line_iter)
    }
}
