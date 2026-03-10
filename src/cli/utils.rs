// Plik: src/cli/utils.rs
use crate::cli::args::{CliUnitSystem, OutputType, SharedTaskArgs};
use lib::fn_filespath::Task;
use lib::fn_weight::{UnitSystem, WeightConfig};

pub fn collect_tasks(args: &SharedTaskArgs) -> Vec<Task<'_>> {
    let mut tasks = Vec::new();

    for t_str in &args.task {
        tasks.push(parse_inline_task(t_str));
    }

    if tasks.is_empty() && args.tasks.is_none() {
        let mut excludes: Vec<&str> = args.exclude.iter().map(|s| s.as_str()).collect();
        if !args.no_default_excludes {
            excludes.extend(vec![
                ".git/",
                "target/",
                "node_modules/",
                ".vs/",
                ".idea/",
                ".vscode/",
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

/// Konwertuje parametry z linii poleceń na strukturę konfiguracyjną API
pub fn build_weight_config(args: &SharedTaskArgs) -> WeightConfig {
    let system = match args.weight_system {
        CliUnitSystem::Decimal => UnitSystem::Decimal,
        CliUnitSystem::Binary => UnitSystem::Binary,
        CliUnitSystem::Both => UnitSystem::Both,
        CliUnitSystem::None => UnitSystem::None,
    };

    WeightConfig {
        system,
        precision: args.weight_precision.max(3), // Minimum 3 znaki na liczbę
        show_for_dirs: !args.no_dir_weight,
        show_for_files: !args.no_file_weight,
        dir_sum_included: !args.real_dir_weight, // Domyślnie sumujemy tylko ujęte w filtrach
    }
}
