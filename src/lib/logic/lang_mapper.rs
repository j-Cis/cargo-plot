pub struct LangMapper;

impl LangMapper {
	/// [POL]: Czarna lista rozszerzeń zabezpieczająca przed próbą odczytu plików binarnych.
	pub fn is_blacklisted(ext: &str) -> bool {
		let e = ext.to_lowercase();
		matches!(
			e.as_str(),
			// GRAFIKA
			"png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "svg" | "webp" | "tiff" | "tif" | "heic" | "psd" | "ai" | 
            // BINARKI
            "exe" | "dll" | "so" | "dylib" | "bin" | "wasm" | "pdb" | "rlib" | "rmeta" | "lib" | "o" | "a" | "obj" | "pch" | "ilk" | "exp" | 
            "jar" | "class" | "war" | "ear" | 
            "pyc" | "pyd" | "pyo" | "whl" | 
            // ARCHIWA
            "zip" | "tar" | "gz" | "tgz" | "7z" | "rar" | "bz2" | "xz" | "iso" | "dmg" | "pkg" | "apk" | 
            // BAZY / DOKUMENTY / FONTY
            "sqlite" | "sqlite3" | "db" | "db3" | "mdf" | "ldf" | "rdb" | 
            "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "odt" | "ods" | "odp" | 
            "woff" | "woff2" | "ttf" | "eot" | "otf" | 
            // MEDIA
            "mp3" | "mp4" | "avi" | "mkv" | "wav" | "flac" | "ogg" | "m4a" | "mov" | "wmv" | "flv"
		)
	}

	/// [POL]: Zwraca identyfikator języka dla bloków kodu w DocMarkdown.
	pub fn get_md_lang(ext: &str) -> &'static str {
		match ext.to_lowercase().as_str() {
			"rs" => "rust",
			"toml" => "toml",
			"slint" => "slint",
			"md" => "markdown",
			"json" => "json",
			"yaml" | "yml" => "yaml",
			"html" => "html",
			"css" => "css",
			"js" => "javascript",
			"ts" => "typescript",
			_ => "text",
		}
	}
}
