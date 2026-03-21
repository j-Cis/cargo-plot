use crate::interfaces::cli::args::CliArgs;
use cargo_plot::addon::TimeTag;
use cargo_plot::core::path_matcher::stats::ShowMode;
use cargo_plot::core::path_store::PathContext;
use cargo_plot::core::path_view::ViewMode;
use cargo_plot::core::save::SaveFile;
use cargo_plot::execute::{self, SortStrategy};
use cargo_plot::i18n::I18n;

// [ENG]: ⚙️ Main execution engine coordinating the scanning and rendering process.
// [POL]: ⚙️ Główny silnik wykonawczy koordynujący proces skanowania i renderowania.
pub fn run(args: CliArgs) {
    // [ENG]: 📝 Reconstructs the command string for the footer.
    // [POL]: 📝 Odtwarza ciąg komendy dla stopki.
    let i18n = I18n::new(args.lang);
    let is_case_sensitive = !args.ignore_case;
    let sort_strategy: SortStrategy = args.sort.into();
    let view_mode: ViewMode = args.view.into();

    // [ENG]: 🎚️ Determines the display mode based on include (-m) and exclude (-x) flags.
    // [POL]: 🎚️ Ustala tryb wyświetlania na podstawie flag włączania (-m) i wykluczania (-x).
    let show_mode = match (args.include, args.exclude) {
        (true, false) => ShowMode::Include,
        (false, true) => ShowMode::Exclude,
        _ => ShowMode::Context,
    };

    // [ENG]: 🚀 Executes the core matching logic.
    // [POL]: 🚀 Wykonuje główną logikę dopasowywania.
    let stats = execute::execute(
        &args.enter_path,
        &args.patterns,
        is_case_sensitive,
        sort_strategy,
        show_mode,
        view_mode,
        args.no_root,
        args.info,
        args.no_emoji,
        &i18n,
        |_| {},
        |_| {},
    );

    // [ENG]: 🖥️ Renders the output to the terminal with ANSI colors.
    // [POL]: 🖥️ Renderuje wynik do terminala z użyciem kolorów ANSI.
    let output_str_cli = stats.render_output(view_mode, show_mode, args.info, true);
    print!("{}", output_str_cli);

    // [ENG]: 💾 Handles file saving if address or archive flags are active.
    // [POL]: 💾 Obsługuje zapis do plików, jeśli aktywne są flagi adresu lub archiwum.
    if args.save_address || args.save_archive {
        let tag = TimeTag::now();

        // [ENG]: 📄 Renders plain text for Markdown output.
        // [POL]: 📄 Renderuje czysty tekst dla wyjścia w formacie Markdown.
        let output_str_txt_m = stats.render_output(view_mode, ShowMode::Include, args.info, false);
        let output_str_txt_x = stats.render_output(view_mode, ShowMode::Exclude, args.info, false);

        // [ENG]: 📂 Resolves the output directory path to .cargo-plot/ by default.
        // [POL]: 📂 Rozwiązuje ścieżkę katalogu wyjściowego (domyślnie na .cargo-plot/).
        let resolve_dir = |val: &Option<String>, base_path: &str| -> String {
            let is_auto = val
                .as_ref()
                .map_or(true, |v| v.trim().is_empty() || v == "AUTO");
            if is_auto {
                let mut b = base_path.replace('\\', "/");
                if !b.ends_with('/') {
                    b.push('/');
                }
                format!("{}.cargo-plot/", b)
            } else {
                let mut p = val.as_ref().unwrap().replace('\\', "/");
                if !p.ends_with('/') {
                    p.push('/');
                }
                p
            }
        };

        let output_dir = resolve_dir(&args.dir_out, &args.enter_path);

        // [ENG]: 📝 Saves the path structure (address).
        // [POL]: 📝 Zapisuje strukturę ścieżek (adres).
        if args.save_address {
            if args.include || (!args.include && !args.exclude) {
                let filepath = format!("{}plot-address_{}_M.md", output_dir, tag);
                let cmd_m = args.to_command_string(true, false, true, false);
                SaveFile::paths(
                    &output_str_txt_m,
                    &filepath,
                    &tag,
                    args.by,
                    &i18n,
                    &cmd_m,
                    &args.enter_path,
                );
            }
            if args.exclude || (!args.include && !args.exclude) {
                let filepath = format!("{}plot-address_{}_X.md", output_dir, tag);
                let cmd_x = args.to_command_string(false, true, true, false);
                SaveFile::paths(
                    &output_str_txt_x,
                    &filepath,
                    &tag,
                    args.by,
                    &i18n,
                    &cmd_x,
                    &args.enter_path,
                );
            }
        }

        // [ENG]: 📦 Saves the full file contents (archive).
        // [POL]: 📦 Zapisuje pełną zawartość plików (archiwum).
        if args.save_archive {
            if let Ok(ctx) = PathContext::resolve(&args.enter_path) {
                if args.include || (!args.include && !args.exclude) {
                    let filepath = format!("{}plot-archive_{}_M.md", output_dir, tag);
                    let cmd_m = args.to_command_string(true, false, false, true);
                    SaveFile::codes(
                        &output_str_txt_m,
                        &stats.m_matched.paths,
                        &ctx.entry_absolute,
                        &filepath,
                        &tag,
                        args.by,
                        &i18n,
                        &cmd_m,
                        &args.enter_path,
                    );
                }
                if args.exclude || (!args.include && !args.exclude) {
                    let filepath = format!("{}plot-archive_{}_X.md", output_dir, tag);
                    let cmd_x = args.to_command_string(false, true, false, true);
                    SaveFile::codes(
                        &output_str_txt_x,
                        &stats.x_mismatched.paths,
                        &ctx.entry_absolute,
                        &filepath,
                        &tag,
                        args.by,
                        &i18n,
                        &cmd_x,
                        &args.enter_path,
                    );
                }
            }
        }
    }

    // [ENG]: 📊 Prints summary statistics if info flag is active.
    // [POL]: 📊 Wyświetla statystyki podsumowujące, jeśli aktywna jest flaga info.
    if args.info {
        println!("---------------------------------------");
        println!(
            "{}",
            i18n.cli_summary_matched(stats.m_size_matched, stats.total)
        );
        println!(
            "{}",
            i18n.cli_summary_rejected(stats.x_size_mismatched, stats.total)
        );
    } else {
        println!("---------------------------------------");
    }
}
