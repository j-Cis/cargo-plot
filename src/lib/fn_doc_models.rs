use crate::fn_filespath::Task;
use crate::fn_weight::WeightConfig;

/// Struktura definiująca jedno zadanie generowania pliku Markdown
pub struct DocTask<'a> {
    pub output_filename: &'a str,
    pub insert_tree: &'a str, // "dirs-first", "files-first", "with-out"
    pub id_style: &'a str,    // "id-tag", "id-num", "id-non"
    pub tasks: Vec<Task<'a>>,
    pub weight_config: WeightConfig, // Nowe pole
    pub watermark: &'a str,
    pub command_str: Option<String>,
    pub suffix_stamp: bool,
    pub title_file: &'a str,
    pub title_file_with_path: bool,
}
