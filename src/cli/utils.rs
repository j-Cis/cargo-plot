// Plik: src/cli/utils.rs
use crate::cli::args::{OutputType, SharedTaskArgs};
use lib::fn_filespath::Task;

pub fn collect_tasks(args: &SharedTaskArgs) -> Vec<Task<'_>> {
    let mut tasks = Vec::new();

    for t_str in &args.task {
        tasks.push(parse_inline_task(t_str));
    }

    if tasks.is_empty() && args.tasks.is_none() {
        let mut excludes: Vec<&str> = args.exclude.iter().map(|s| s.as_str()).collect();
        if !args.no_default_excludes {
            excludes.extend(vec![
                ".git/", "target/", "node_modules/", ".vs/", ".idea/", ".vscode/",
            ]);
        }

        tasks.push(Task {
            path_location: &args.path,
            path_exclude: excludes,
            path_include_only: args.include_only.iter().map(|s| s.as_str()).collect(),
            filter_files: args.filter_files.iter().map(|s| s.as_str()).collect(),
            output_type: match args.r#type {
                OutputType::Dirs => "dirs",
                OutputType::Files => "files",
                _ => "dirs_and_files",
            },
        });
    }

    tasks
}

fn parse_inline_task(input: &str) -> Task<'_> {
    let mut task = Task::default();
    let parts = input.split(',');
    for part in parts {
        let kv: Vec<&str> = part.split('=').collect();
        if kv.len() == 2 {
            match kv[0] {
                "loc" => task.path_location = kv[1],
                "inc" => task.path_include_only.push(kv[1]),
                "exc" => task.path_exclude.push(kv[1]),
                "fil" => task.filter_files.push(kv[1]),
                "out" => task.output_type = kv[1],
                _ => {}
            }
        }
    }
    task
}