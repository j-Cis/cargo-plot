use crate::fn_filespath::filespath;
use crate::fn_doc_models::DocTask;
use crate::fn_doc_id::generate_ids;
use crate::fn_doc_write::write_md;
use crate::fn_filestree::filestree; 
use crate::fn_plotfiles::plotfiles_txt; 
use crate::fn_datestamp::datestamp_now;
use std::fs;
use std::io;

pub fn generate_docs(
    doc_tasks: Vec<DocTask>, 
    output_dir: &str
) -> io::Result<()> {
    fs::create_dir_all(output_dir)?;

    for doc_task in doc_tasks {
        // Generujemy jeden wspólny znacznik czasu dla zadania
        let stamp = datestamp_now();
        
        // Budujemy nazwę pliku np. "code__2026Q1D068W11_Mon09Mar_212719746.md"
        let out_file = format!("{}__{}.md", doc_task.output_filename, stamp);
        let out_path = format!("{}/{}", output_dir, out_file);
        
        // 1. Zbieramy ścieżki
        let paths = filespath(&doc_task.tasks);

        // 2. Generowanie tekstu drzewa
        let tree_text = if doc_task.insert_tree != "with-out" {
            let tree_nodes = filestree(paths.clone(), doc_task.insert_tree);
            let txt = plotfiles_txt(&tree_nodes, "", None);
            Some(txt)
        } else {
            None
        };
        
        // 3. Nadajemy identyfikatory
        let id_map = generate_ids(&paths);
        
        // 4. Przekazujemy styl ID do funkcji zapisu
        write_md(&out_path, &paths, &id_map, tree_text, &stamp, doc_task.id_style)?;
        
        // Możemy wydrukować info o POJEDYNCZYM wygenerowanym pliku
        println!(" [+] Wygenerowano raport: {}", out_path);
    }
    
    Ok(())
}