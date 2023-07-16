use crate::{analyze_c::AnalyzeC, analyze_java::AnalyzeJava, LineIter, LogEvent};

pub async fn analyze(mut line_iter: LineIter) -> Vec<String> {
    let mut analyze_log = vec![];
    while let Some(line) = line_iter.next_line().await.unwrap() {
        if let Some(mut analyzer) = AnalyzeC::new(&line) {
            let compile_duration = analyzer.get_duration(line_iter).await;
            if let Some(compile_duration) = compile_duration.0 {
                analyze_log.push(format!(
                    "compile {} duration {}s, about {:.2}min",
                    analyzer.target_name(),
                    compile_duration,
                    compile_duration / 60.
                ));
            }
            line_iter = compile_duration.1;
        } else if let Some(mut analyzer) = AnalyzeJava::new(&line) {
            let compile_duration = analyzer.get_duration(line_iter).await;
            if let Some(compile_duration) = compile_duration.0 {
                analyze_log.push(format!(
                    "compile {} duration {}s, about {:.2}min",
                    analyzer.project_name(),
                    compile_duration,
                    compile_duration / 60.
                ));
            }
            line_iter = compile_duration.1;
        }
    }
    analyze_log
}
