use crate::fn_filespath::Task;

/// Struktura definiująca jedno zadanie generowania pliku Markdown
pub struct DocTask<'a> {
    pub output_filename: &'a str,
    pub insert_tree: &'a str, // "dirs-first", "files-first", "with-out"
    pub id_style: &'a str,    // "id-tag", "id-num", "id-non"
    pub tasks: Vec<Task<'a>>,
}
