// ./src/lib/command/help.rs

pub fn help_for_pattern_syntax_and_semantics() {
    println!(
        r#"
======================================================================
🔎 PATTERN SYNTAX AND SEMANTICS
======================================================================

1. BASICS (GLOB):
   * - Matches any sequence of characters (except directory separators `/`)
   ** - Matches anything including subdirectories (recursive)
   ?        - Matches exactly one character

2. BRACE EXPANSION:
   {{a,b,c}}  - Matches 'a', 'b', or 'c'
   *.{{rs,md}} - Expands to '*.rs' and '*.md'

3. RELATIONAL OPERATORS (CONTEXT RULES):
   @        - Requires a sibling. E.g., `@*.rs` (the .rs file must have an adjacent file/dir with the same base name)
   $        - Orphan. E.g., `$*.rs` (the .rs file MUST NOT have a sibling)
   +        - Deep mode: includes the entire contents of a directory. E.g., `src/+`

4. PARENT MODIFIERS (PARENT INCLUSION):
   &/ OR &\ - Includes all parent directories for the matched file in the
              result. Essential to prevent files from hanging in a void
              when rendering a tree structure. E.g., `*.rs&/`

5. NEGATION:
   !        - Excludes files matching the pattern. E.g., `!*.log`

Usage example:
x-do -p "*.{{rs,toml}}&/" "!tests/**" -m
"#
    );
}

pub fn help_for_automatization_on_config_file() {
    println!(
        r#"
======================================================================
⚙️  AUTOMATIZATION & TOML CONFIGURATION
======================================================================

You can define batch jobs in a TOML configuration file.
Generate a default template using: `x-do -I`
Execute the configuration using:   `x-do -g`

1. JOB METADATA:
   [[job]]
   id = "p1"              # Unique identifier for the job
   name = "snap"          # Optional name
   description = "..."    # Optional description

2. PATTERN & INPUT ([job.pattern]):
   work_path = "."        # Base directory to scan
   ignore_case = false    # Case insensitive matching
   patterns = ["*.rs"]    # Array of match patterns
   mode = "matched"       # "matched" ("m") or "mismatched" ("x")

3. LAYOUT & VIEW ([job.layout]):
   list_instead_tree = false # Disable tree view (flat list)
   sort = "kind"             # Sort criteria (name, size, date, kind, etc.)
   reverse = false           # Reverse sort order
   columns = ["path"]        # Array of columns to display
   ext_icons = false         # Use extended filetype icons

4. TRIMMING & LIMITS ([job.trimming]):
   trim_page = 1          # Start page (requires trim_size to be set)
   trim_size = 20         # Number of items per page (limit)

5. EXPORT OPTIONS ([job.export]):
   save_sotc_at = "./s"   # SOTC (Table & Stats only) output path
   title_sotc = "Title"   # Optional header title for SOTC
   save_cots_at = "./c"   # COTS (Table + Code) output path
   title_cots = "Title"   # Optional header title for COTS

6. RENDER FLAGS ([job.render]):
   hide_stats = false     # Hide scan statistics header
   hide_promo = false     # Hide promotional footer
   quiet = false          # Suppress terminal output

Usage example:
x-do -g                   # Runs the configuration from ./.x-do.toml
x-do -g ./my_config.toml  # Runs the configuration from a specific file
"#
    );
}