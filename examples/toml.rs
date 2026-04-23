

impl AnchoredRuntime {
    pub fn start_at(root_dir: PathBuf) -> Self {
        let exec_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

        let save_dir = PathBuf::from("./target/.cargo-plot");
        let task_file = save_dir.join("task2.toml");

        let runtime = Self {
            exec_dir,
            root_dir,
            save_dir,
            task_file,
        };

        runtime.ensure_task_file();
        runtime
    }
	fn ensure_task_file(&self) {
        if let Some(parent) = self.task_file.parent() {
            let _ = fs::create_dir_all(parent);
        }

        if !self.task_file.exists() {
            let default = r#"
[[job]]
id = "a1"

[[job]]
id = "a2"
"#;

            let _ = fs::write(&self.task_file, default);
        }
    }
	fn has_at_least_one_job(&self) -> bool {
		let content = match fs::read_to_string(&self.task_file) {
			Ok(c) => c,
			Err(_) => return false,
		};

		let parsed: TaskFile = match toml::from_str(&content) {
			Ok(p) => p,
			Err(_) => return false,
		};

		!parsed.job.is_empty()
	}
}




fn main() {
    let x = AnchoredRuntime::start();

    println!("Root: {:?}", x.root);
    println!("Has jobs: {}", x.has_at_least_one_job());
}