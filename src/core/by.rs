use super::super::i18n::I18n;
use std::env;

pub struct BySection;

impl BySection {
    #[must_use]
    pub fn generate(tag: &str, typ: &str, i18n: &I18n) -> String {
        let args: Vec<String> = env::args().collect();
        let command = args.join(" ");

        format!(
            "\n\n---\n---\n\n{}\n\n{}\n\n```bash\n{}\n```\n\n{}\n\n{}\n\n{}\n\n---\n",
            i18n.by_title(typ),
            i18n.by_cmd(),
            command,
            i18n.by_instructions(),
            i18n.by_link(),
            i18n.by_version(tag)
        )
    }
}
