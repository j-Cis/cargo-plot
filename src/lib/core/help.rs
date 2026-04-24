pub fn print_help_for_pattern_syntax_and_semantics() {
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
cargo plot -b -p "*.{{rs,toml}}&/" "!target/**"
"#
	);
}

pub fn print_help_for_toml_config() {
	println!(
		r#"
======================================================================
⚙️  AUTOMATIZATION & TOML CONFIGURATION
======================================================================

You can define batch jobs in a TOML configuration file.
Default path: ./target/.cargo-plot/tasks/CargoPlot.toml

1. JOB METADATA:
   [[job]]
   id = "a1"              # Unique identifier for the job
   description = "..."    # Optional description
   run-mode = ["save"]    # Execution modes: "dry-run", "save", "print-color", etc.

2. EXPLORER ([explorer]):
   workspace-dir = "."    # Base directory to scan
   ignore-case = false    # Case insensitive matching
   patterns = ["*.rs"]    # Array of match patterns
   parts = ["MD", "MF"]   # Matched Dirs, Matched Files, etc. (MD, MF, XD, XF)

3. EXPORT OPTIONS ([export]):
   out-dir = "./out/"     # Output directory for reports
   title = "Title"        # Report header title
   name = "MyReport"      # Output file base name
   name-is-first = false  # Put name at the beginning of the file name
   save-separately = true # Split INDEX (SOTC) and FILES (COTS)

4. ATTRIBUTES & FORMATTING ([attributes]):
   select = ["date", "time", "size", "path", "item"] # Columns to show
   for-item = ["list-tree", "icons-more"]            # Item styling (list-tree, list-flat, icons-hide...)
   for-date = "%Y-%m-%d"                             # Date formatting
   for-time = "%H:%M:%S"                             # Time formatting
   for-size = "decimal"                              # "decimal" or "binary"

5. PILE & SORT ([tuples.pile] & [tuples.sort]):
   [tuples.pile]
   type = "name"          # Grouping strategy ("name", "exte")
   dir-first = true       # Dirs first in groups
   same-name-dirs-and-files-nearby = true

   [tuples.sort]
   type = "name"          # Sort by ("date", "size", "path", "name")
   reverse = false
   string-strategy = ["AaZz", "Num", "Spec"]

Usage examples:
cargo plot -l                 # Runs the configuration from default TOML
cargo plot -l ./custom.toml   # Runs the configuration from a specific file
"#
	);
}
