use regex::Regex;

use crate::{LineIter, LogEvent, Seconds};

pub(crate) struct AnalyzeJava {
    project_name: String,
}

impl AnalyzeJava {
    pub fn new(start_log: &str) -> Option<Self> {
        if Self::start_re().is_match(start_log) {
            let project_name = Self::get_project_name(start_log).unwrap();
            Some(Self {
                project_name,
            })
        } else {
            None
        }
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
        if start_re.is_match(log) {
            Some(start_re.captures(log).unwrap()[1].trim().to_string())
        } else {
            None
        }
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
            if end_re.is_match(&line) {
                return (
                    Some(end_re.captures(&line).unwrap()[1].trim().parse::<Seconds>().unwrap()),
                    line_iter,
                );
            }
        }
        (None, line_iter)
    }
}
