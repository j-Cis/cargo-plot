use crate::fn_filestree::FileNode;
use colored::*;

/// Zestaw znaków używanych do rysowania gałęzi drzewa.
#[derive(Debug, Clone)]
pub struct TreeStyle {
    // Foldery (d)
    pub dir_last_with_children: String, // └──┬
    pub dir_last_no_children: String,   // └───
    pub dir_mid_with_children: String,  // ├──┬
    pub dir_mid_no_children: String,    // ├───

    // Pliki (f)
    pub file_last: String, // └──
    pub file_mid: String,  // ├──

    // Wcięcia dla kolejnych poziomów (i)
    pub indent_last: String, // "   " (3 spacje)
    pub indent_mid: String,  // "│  " (kreska + 2 spacje)
}

impl Default for TreeStyle {
    fn default() -> Self {
        Self {
            dir_last_with_children: "└──┬".to_string(),
            dir_last_no_children: "└───".to_string(),
            dir_mid_with_children: "├──┬".to_string(),
            dir_mid_no_children: "├───".to_string(),

            file_last: "└──•".to_string(),
            file_mid: "├──•".to_string(),

            indent_last: "   ".to_string(),
            indent_mid: "│  ".to_string(),
        }
    }
}

/// Prywatna funkcja pomocnicza, która odwala całą powtarzalną robotę.
fn plot(nodes: &[FileNode], indent: &str, s: &TreeStyle, use_color: bool) -> String {
    let mut result = String::new();

    for (i, node) in nodes.iter().enumerate() {
        let is_last = i == nodes.len() - 1;
        let has_children = !node.children.is_empty();

        // 1. Wybór odpowiedniego znaku gałęzi
        let branch = if node.is_dir {
            match (is_last, has_children) {
                (true, true) => &s.dir_last_with_children,
                (false, true) => &s.dir_mid_with_children,
                (true, false) => &s.dir_last_no_children,
                (false, false) => &s.dir_mid_no_children,
            }
        } else if is_last {
            &s.file_last
        } else {
            &s.file_mid
        };

        // 2. Formatowanie konkretnej linii (z kolorami lub bez)
        let line = if use_color {
            if node.is_dir {
                format!(
                    "{}{} {}{}/\n",
                    indent.green(),
                    branch.green(),
                    node.icon,
                    node.name.truecolor(200, 200, 50)
                )
            } else {
                format!(
                    "{}{} {}{}\n",
                    indent.green(),
                    branch.green(),
                    node.icon,
                    node.name.white()
                )
            }
        } else {
            format!("{}{} {} {}\n", indent, branch, node.icon, node.name)
        };

        result.push_str(&line);

        // 3. Rekurencja dla dzieci z wyliczonym nowym wcięciem
        if has_children {
            let new_indent = if is_last {
                format!("{}{}", indent, s.indent_last)
            } else {
                format!("{}{}", indent, s.indent_mid)
            };
            result.push_str(&plot(&node.children, &new_indent, s, use_color));
        }
    }

    result
}

/// GENEROWANIE PLAIN TEXT / MARKDOWN
pub fn plotfiles_txt(nodes: &[FileNode], indent: &str, style: Option<&TreeStyle>) -> String {
    let default_style = TreeStyle::default();
    let s = style.unwrap_or(&default_style);

    plot(nodes, indent, s, false)
}

/// GENEROWANIE KOLOROWANEGO ASCII DO CLI
pub fn plotfiles_cli(nodes: &[FileNode], indent: &str, style: Option<&TreeStyle>) -> String {
    let default_style = TreeStyle::default();
    let s = style.unwrap_or(&default_style);

    plot(nodes, indent, s, true)
}
