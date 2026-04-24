pub mod lib {
	pub mod core {
		mod help;
		pub use help::{print_help_for_pattern_syntax_and_semantics, print_help_for_toml_config};
		mod job_presets;
		pub use job_presets::{
			DEFAULT_DIR_FIRST,
			DEFAULT_FOR_DATE,
			DEFAULT_FOR_SIZE,
			DEFAULT_FOR_TIME,
			DEFAULT_IGNORE_CASE,
			DEFAULT_MIRROR,
			DEFAULT_NAME,
			DEFAULT_NAME_IS_FIRST,
			DEFAULT_REVERSE,
			DEFAULT_SAME_NAME_DIRS_AND_FILES_NEARBY,
			DEFAULT_SAVE_SEPARATELY,
			DEFAULT_TITLE,
			TOML_DEFAULT_MINIMAL,
			default_attributes_select,
			default_config_path,
			default_explorer_parts,
			default_explorer_patterns,
			default_for_item,
			default_out_dir,
			default_run_modes,
			default_string_strategy,
			default_workspace_dir,
			execution_dir,
		};
		mod job_init;
		pub use job_init::{AnchoredRuntime, start, start_blank};
	}
	pub mod logic {
		mod tag_time;
		pub use tag_time::{TagTime, tag_from_time, tag_from_time_now, tag_to_time};
		mod fs_file;
		pub use fs_file::{file_backup, file_ensure, file_remove, file_save_force, file_save_safe, file_save_safe_if_changed};
		mod fs_scanner;
		pub use fs_scanner::{
			ScanAsMetadataNode,
			//PathNode,
			//PattEnvIndex,
			ScanMatchLabel,
			ScanMatchedDir,
			ScanMatchedFile,
			ScanMismatchedDir,
			ScanMismatchedFile,
			ScanNodeDirScanned,
			ScanNodeFileScanned,
			ScanNodeIs,
			ScanNodeScanned,
			ScanPartition,
			ScanPartitionScanned,
			ScanStatsPartitioning,
			ScanStatsTreeFsScanned,
			scan_file_is_binary,
		};
		mod fs_anchored_datum;
		pub use fs_anchored_datum::{AnchoredPathsDatum, PathNode};
		mod path_context;
		pub use path_context::PathContext;
		mod path_patterns;
		pub use path_patterns::{PattEnvIndex, PattExp, PattRaw, PatternsQueries};
	}
	pub mod schema {
		mod job_raw_toml;
		pub use job_raw_toml::{
			RawTomlFileJobs,
			RawTomlJob,
			RawTomlJobAttributes,
			RawTomlJobAttributesOptions,
			RawTomlJobExplorer,
			RawTomlJobExport,
			RawTomlJobPileMode,
			RawTomlJobSortNum,
			RawTomlJobSortTex,
			RawTomlJobStringStrategy,
			RawTomlJobTuples,
			RawTomlJobTuplesPile,
			RawTomlJobTuplesSort,
			RawTomlJobs,
			RawTomlValidJob,
			RawTomlValidJobs,
			SharedJobAttributeToSelect,
			SharedJobOptForAttrItem,
			SharedJobOptForAttrSize,
			SharedJobPart,
			SharedJobRunMode,
			SharedJobStringMode,
		};
		mod job_raw_cli;
		pub use job_raw_cli::{CargoCliRoot, RawCliJob};
		mod job_ready;
		pub use job_ready::{
			ReadyJob,
			ReadyJobAttributes,
			ReadyJobExplorer,
			ReadyJobExport,
			ReadyJobPileMode,
			ReadyJobSortNum,
			ReadyJobSortTex,
			ReadyJobTuples,
			ReadyJobTuplesPile,
			ReadyJobTuplesSort,
		};
		mod job_pipeline;
		pub use job_pipeline::{PipelineJobRow, PipelineJobTab};
	}
	pub mod mapping {
		mod prepare_job_raw_toml_to_ready;
		pub use prepare_job_raw_toml_to_ready::prepare;
		mod middleware_translate_to_job_raw_from_cli;
		pub use middleware_translate_to_job_raw_from_cli::translate;
		mod engine_cli;
		pub use engine_cli::route_and_execute;
		mod mapper_lang_type;
		pub use mapper_lang_type::LangMapper;
	}
	pub mod pipelines {
		mod step1;
		pub use step1::engine_step1_scanner;
		mod step2;
		pub use step2::{FormattedRow, engine_step2_data_formater, render_table};
		mod step6;
		pub use step6::engine_step6_data_save;
		mod step7;
		pub use step7::engine_step7_data_view;
		//pub mod step3;
		//pub mod step6;
	}
}
