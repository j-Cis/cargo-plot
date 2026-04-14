// Wewnątrz impl TabSpec
pub fn from_config(cfg: &ConfigLayout, spec: &ConfigSpec) -> Result<Self, String> {
    let mut columns = Vec::new();
    for col_str in &cfg.columns {
        columns.push(TabColumn::parse(col_str)?);
    }

    // Używamy pól sort_by i sort_order z modelu
    let mut tab = Self::default()
        .sort(
            TabSortBy::parse(&cfg.sort_by)?,
            // Tu decydujemy: albo flaga reverse, albo parsujemy sort_order
            if cfg.reverse { TabSortOrder::Desc } else { TabSortOrder::Asc },
            TabPathStructure::parse(&cfg.structure)?
        )
        .columns(&columns)
        .more_icons(cfg.more_icons);

    tab.trim_page = cfg.trim_page;

    // Trimming jest teraz wewnątrz spec.trimming
    if let Some(trim) = &spec.trimming {
        if let Some(size) = trim.page_size {
            tab = tab.trim(size, trim.show_page);
        }
    }

    Ok(tab)
}