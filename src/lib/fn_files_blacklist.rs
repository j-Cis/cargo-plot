// src/lib/fn_files_blacklist.rs

/// Sprawdza, czy podane rozszerzenie pliku należy do czarnej listy (pliki binarne, graficzne, media, archiwa).
/// Zwraca `true`, jeśli plik powinien zostać pominięty podczas wczytywania zawartości tekstowej.
pub fn is_blacklisted_extension(ext: &str) -> bool {
    let binary_extensions = [
        // --------------------------------------------------
        // GRAFIKA I DESIGN
        // --------------------------------------------------
        "png", "jpg", "jpeg", "gif", "bmp", "ico", "svg", "webp", "tiff", "tif", "heic", "psd",
        "ai",
        // --------------------------------------------------
        // BINARKI, BIBLIOTEKI I ARTEFAKTY KOMPILACJI
        // --------------------------------------------------
        // Rust / Windows / Linux / Mac
        "exe", "dll", "so", "dylib", "bin", "wasm", "pdb", "rlib", "rmeta", "lib",
        // C / C++
        "o", "a", "obj", "pch", "ilk", "exp", // Java / JVM
        "jar", "class", "war", "ear", // Python
        "pyc", "pyd", "pyo", "whl",
        // --------------------------------------------------
        // ARCHIWA I PACZKI
        // --------------------------------------------------
        "zip", "tar", "gz", "tgz", "7z", "rar", "bz2", "xz", "iso", "dmg", "pkg", "apk",
        // --------------------------------------------------
        // DOKUMENTY, BAZY DANYCH I FONTY
        // --------------------------------------------------
        // Bazy danych
        "sqlite", "sqlite3", "db", "db3", "mdf", "ldf", "rdb", // Dokumenty Office / PDF
        "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "odt", "ods", "odp",
        // Fonty
        "woff", "woff2", "ttf", "eot", "otf",
        // --------------------------------------------------
        // MEDIA (AUDIO / WIDEO)
        // --------------------------------------------------
        "mp3", "mp4", "avi", "mkv", "wav", "flac", "ogg", "m4a", "mov", "wmv", "flv",
    ];

    binary_extensions.contains(&ext)
}
