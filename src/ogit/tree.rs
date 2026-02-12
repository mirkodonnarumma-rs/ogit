/// ```text
/// funzione build_tree_from_dir(path):
///
/// inizializza lista entries vuota
///
/// per ogni elemento presente in path:
///     se l'elemento è un file:
///         leggi il contenuto del file
///         crea un oggetto Blob con il contenuto
///         scrivi l'oggetto nello storage e ottieni il relativo hash
///         aggiungi a entries una voce { tipo: Blob, hash, nome }
///
///     altrimenti se l'elemento è una directory:
///         richiama ricorsivamente build_tree_from_dir sulla sottodirectory
///         ottieni l'hash del tree risultante
///         aggiungi a entries una voce { tipo: Tree, hash, nome }
///
/// serializza entries in un payload binario
/// crea un oggetto Tree con il payload
/// scrivi l'oggetto nello storage
/// restituisci l'hash risultante
/// ```

use std::fs::{self, read};
use std::path::Path;

use crate::object::{OObjectId, OObject, TreeEntry};
use crate::store::write_object;

pub fn build_tree_from_dir(store_path: &Path, dir_path: &Path) -> Result<OObjectId, String> {
    let mut entries: Vec<TreeEntry> = Vec::new();
    
    // 1. Leggi contenuto directory
    let read_dir = fs::read_dir(dir_path)
        .map_err(|e| format!("Failed to read dir: {}", e))?;
    
    // 2. Itera su ogni elemento
    for entry_result in read_dir {
        let entry = entry_result
            .map_err(|e| format!("Failed to read entry: {}", e))?;
        
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        
        // 3. Ignora .ogit
        if name == ".ogit" {
            continue;
        }
        
        // 4. Gestisci file vs directory
        if path.is_file() {
            // Leggo il contenuto del file
            let content = read(&path);
            // Creo un blob
            let blob = OObject::new_blob(content.unwrap());
            // Scrivo il blob
            let hash = write_object(store_path, &blob);
            // Inserisco nelle entries
            entries.push(TreeEntry { kind: crate::object::OObjectType::Blob, hash: hash.unwrap(), name  })
            
        } else if path.is_dir() {
            // TODO: ricorsione e aggiungi entry
            let hash = build_tree_from_dir(store_path, &path);
            entries.push(TreeEntry { kind: crate::object::OObjectType::Tree, hash: hash.unwrap(), name  });
        }
    }
    
    // 5. Serializza e salva tree
    let tree_data = TreeEntry::serialize_tree(&entries);
    let tree_obj = OObject::new_tree(tree_data);
    crate::store::write_object(store_path, &tree_obj)
}