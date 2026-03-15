#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    POL,
    ENG,
}

#[derive(Clone, Debug)]
pub struct JobConfig {
    pub title: String,
    pub path_enter: String,
    pub glob_includes: Vec<String>,
    pub glob_excludes: Vec<String>,
    pub file_types: Vec<String>,
    pub dirs_include_empty: bool,
    pub dirs_only: bool,
    pub dirs_keep_excluded_as_empty_to_depth: u32,
    pub path_include_parent_file: bool,
}

impl Default for JobConfig {
    fn default() -> Self {
        Self {
            title: "default".to_string(),
            path_enter: "./src/".to_string(),
            glob_includes: vec!["./Cargo.toml".to_string()],
            glob_excludes: vec![],
            file_types: vec!["*.rs".to_string()],
            dirs_include_empty: true,
            dirs_only: false,
            dirs_keep_excluded_as_empty_to_depth: 0,
            path_include_parent_file: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SortOrder {
    FilesFirst,
    DirsFirst,
    Alphanumeric,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeBase {
    Base1024,
    Base1000,
}

#[derive(Clone, Debug)]
pub struct PathsStructStyleConfig {
    pub sort: SortOrder,
    pub size_files: bool,
    pub size_dirs: bool,
    pub size_dirs_real: bool,
    pub size_base: SizeBase,
    pub precision: u8,
}

impl Default for PathsStructStyleConfig {
    fn default() -> Self {
        Self {
            sort: SortOrder::FilesFirst,
            size_files: true,
            size_dirs: false,
            size_dirs_real: false,
            size_base: SizeBase::Base1024,
            precision: 5,
        }
    }
}

#[derive(Clone, Debug)]
pub struct OutputSaveConfig {
    pub save_file_f: bool, // Struktura (f)
    pub save_file_d: bool, // Dokumentacja (d)
    pub save_file_c: bool, // Komenda (c)
    pub add_struct_to_doc: bool,
    pub add_cmd_to_struct: bool,
    pub add_cmd_to_doc: bool,
    pub auto_section_num: bool,
    pub timestamp_in_file: bool,
    pub timestamp_in_filename: bool,
    pub self_promo: bool,
    pub output_folder: String,
    pub section_prefix: String,
    pub doc_title: String,
}

impl Default for OutputSaveConfig {
    fn default() -> Self {
        Self {
            save_file_f: true,
            save_file_d: false,
            save_file_c: false,
            add_struct_to_doc: true,
            add_cmd_to_struct: true,
            add_cmd_to_doc: true,
            auto_section_num: true,
            timestamp_in_file: true,
            timestamp_in_filename: false,
            self_promo: false,
            output_folder: "./other/".to_string(),
            section_prefix: "File-".to_string(),
            doc_title: "".to_string(),
        }
    }
}

pub struct StateTui {
    pub lang: Lang,
    pub jobs: Vec<JobConfig>,
    pub struct_config: PathsStructStyleConfig,
    pub output_config: OutputSaveConfig,
}

impl StateTui {
    pub fn new() -> Self {
        Self {
            lang: Lang::POL,
            jobs: Vec::new(),
            struct_config: PathsStructStyleConfig::default(),
            output_config: OutputSaveConfig::default(),
        }
    }

    // Teraz add_job przyjmuje CAŁĄ konfigurację, a nie tylko stringa
    pub fn add_job(&mut self, mut job: JobConfig) {
        let base_title = job.title.clone();
        let mut title = base_title.clone();
        let mut counter = 1;

        // Magia sufiksów (_1, _2...)
        while self.jobs.iter().any(|j| j.title == title) {
            title = format!("{}_{}", base_title, counter);
            counter += 1;
        }

        job.title = title;
        self.jobs.push(job);
    }
}
