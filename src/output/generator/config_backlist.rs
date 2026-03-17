// [EN]: Security mechanisms to prevent processing non-text or binary files.
// [PL]: Mechanizmy bezpieczeństwa zapobiegające przetwarzaniu plików nietekstowych lub binarnych.

/// [EN]: Checks if a file extension is on the list of forbidden binary types.
/// [PL]: Sprawdza, czy rozszerzenie pliku znajduje się na liście zabronionych typów binarnych.
#[must_use]
pub fn is_blacklisted_extension(ext: &str) -> bool {
    // [EN]: Standardize to lowercase to handle e.g., .PNG and .png the same way.
    // [PL]: Standaryzacja do małych liter, aby traktować np. .PNG i .png tak samo.
    let e = ext.to_lowercase();

    // [EN]: We use matches! macro for better performance than array.contains().
    // [PL]: Używamy makra matches! dla lepszej wydajności niż array.contains().
    matches!(
        e.as_str(),
        // --------------------------------------------------
        // GRAFIKA I DESIGN
        // --------------------------------------------------
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "svg" | "webp" | "tiff" | "tif" | "heic" | "psd" | 
        "ai" | 
        // --------------------------------------------------
        // BINARKI | BIBLIOTEKI I ARTEFAKTY KOMPILACJI
        // --------------------------------------------------
        // Rust / Windows / Linux / Mac
        "exe" | "dll" | "so" | "dylib" | "bin" | "wasm" | "pdb" | "rlib" | "rmeta" | "lib" | 
        // C / C++
        "o" | "a" | "obj" | "pch" | "ilk" | "exp" | // Java / JVM
        "jar" | "class" | "war" | "ear" | // Python
        "pyc" | "pyd" | "pyo" | "whl" | 
        // --------------------------------------------------
        // ARCHIWA I PACZKI
        // --------------------------------------------------
        "zip" | "tar" | "gz" | "tgz" | "7z" | "rar" | "bz2" | "xz" | "iso" | "dmg" | "pkg" | "apk" | 
        // --------------------------------------------------
        // DOKUMENTY | BAZY DANYCH I FONTY
        // --------------------------------------------------
        // Bazy danych
        "sqlite" | "sqlite3" | "db" | "db3" | "mdf" | "ldf" | "rdb" | // Dokumenty Office / PDF
        "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "odt" | "ods" | "odp" | 
        // Fonty
        "woff" | "woff2" | "ttf" | "eot" | "otf" | 
        // --------------------------------------------------
        // MEDIA (AUDIO / WIDEO)
        // --------------------------------------------------
        "mp3" | "mp4" | "avi" | "mkv" | "wav" | "flac" | "ogg" | "m4a" | "mov" | "wmv" | "flv"
    )
}
