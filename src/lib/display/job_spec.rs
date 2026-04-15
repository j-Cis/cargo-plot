use crate::lib::logic::{JobMode, JobSpec, TabSortOrder};
use super::Color;

impl std::fmt::Display for JobSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name.as_deref().unwrap_or("None");
        let desc = self.description.as_deref().unwrap_or("None");
		
		let quiet_status = if self.quiet_work { "🔇 (Quiet)" } else { "🔊 (Verbose)" };
		
        // Helpery do kolorowania zmiennych, aby utrzymać czystość makr
        let c_bool = |b: bool| Color::size(&b.to_string()).to_string();
        let c_num = |n: usize| Color::size(&n.to_string()).to_string();

		println!("{}", "░".repeat(80));
        // --- NAGŁÓWEK ---
        writeln!(f, "【ID】🔖 [{}] | {} | {}", Color::num(&self.id), Color::folder(name), quiet_status)?;
        writeln!(f, "  │   📜 {}", Color::border(desc))?;
        writeln!(f, "  │")?;

        // --- SEKCJA SCAN ---
        writeln!(f, "  ├──【SCAN】")?;
        writeln!(f, "  │    ├── 🎯 \"{}\"", Color::folder(&self.scan.work_path))?; 
        writeln!(f, "  │    └── 🔎 | 🚫🔠 [{}]", c_bool(self.scan.ignore_case))?;
        
        let pats_len = self.scan.patterns.len();
        for (i, pat) in self.scan.patterns.iter().enumerate() {
            let branch = if i == pats_len - 1 { "└──" } else { "├──" };
            writeln!(f, "  │        {} \"{}\"", branch, Color::border(pat))?;
        }
        
        writeln!(f, "  │")?;

        // --- SEKCJA EXPORT ---
        writeln!(f, "  ├──【EXPORT】")?;
        let has_sotc = self.save_sotc_at.is_some();
        let sotc_path = self.save_sotc_at.as_deref().unwrap_or("");
        let sotc_title = self.title_sotc.as_deref().unwrap_or("");
        writeln!(f, "  │    ├──【SOTC】🗃️  [{}] | 💾 \"{}\"", c_bool(has_sotc), Color::folder(sotc_path))?;
        writeln!(f, "  │    │    └── 🔤 \"{}\"", Color::border(sotc_title))?;
        
        let has_cots = self.save_cots_at.is_some();
        let cots_path = self.save_cots_at.as_deref().unwrap_or("");
        let cots_title = self.title_cots.as_deref().unwrap_or("");
        writeln!(f, "  │    └──【COTS】📚 [{}] | 💾 \"{}\"", c_bool(has_cots), Color::folder(cots_path))?;
        writeln!(f, "  │         └── 🔤 \"{}\"", Color::border(cots_title))?;
        
        writeln!(f, "  │")?;

        // --- SEKCJA TABLE ---
        writeln!(f, "  ├──【TABLE】")?;
        let cols = self.table.columns.iter().map(|c| format!("{:?}", c)).collect::<Vec<_>>().join(", ");
        let struct_name = format!("{:?}", self.table.structure);
        writeln!(f, "  │    ├── 🧩 [{}] | 🏗️  [{}] | ➕ [{}]", Color::size(&cols), Color::size(&struct_name), c_bool(self.table.more_icons))?;
        
        let sort_by_str = format!("{:?}", self.table.sort_by).to_lowercase();
        let sort_order_str = format!("{:?}", self.table.sort_order).to_lowercase();
        let is_reverse = self.table.sort_order == TabSortOrder::Desc;
        writeln!(f, "  │    └── 📶 (\"{}\", \"{}\") | ↩️  [{}]", Color::size(&sort_by_str), Color::size(&sort_order_str), c_bool(is_reverse))?;
        
        writeln!(f, "  │")?;

        // --- SEKCJA SPEC ---
        writeln!(f, "  └──【SPEC】")?;
        let is_m = self.mode == JobMode::Matched;
        let is_x = self.mode == JobMode::Mismatched;
        writeln!(f, "       ├── ⚙️ ✔️  Matched [{}] | ⚙️ ✖️  Mismatched [{}]", c_bool(is_m), c_bool(is_x))?;
        writeln!(f, "       ├── 🙈📊 [{}] | 🙈ℹ️  [{}]", c_bool(self.hide_stats), c_bool(self.hide_promo))?;
        
        let is_trim = self.table.trim_size.is_some();
        let t_size = self.table.trim_size.unwrap_or(20);
        let t_page = self.table.trim_page;
        
        write!(f, "       └── ✂️ 📃 [{}] | 📄📐 [{}] | 📄👁️  [{}] ", c_bool(is_trim), c_num(t_size), c_num(t_page))
    }
}