use std::path::Path;
use std::fs::{create_dir_all, read, write};

use crate::object::Commit;

use super::object::{OObject, OObjectId};
use super::hashing_values::{hash_bytes, bytes_to_hex};

pub fn read_object(store_path: &Path, id: &OObjectId) -> Result<OObject, String> {
    /* Algoritmo */
    // Estrai hash hex da OObjectId
    // Splitta: primi 2 char = subdir, resto = filename
    // Costruisci path completo
    // Leggi bytes da file
    // Deserializza in OObject
    // Restituisci
    
    let hash_hex = id.as_str();
    let (subdir, filename) = hash_hex.split_at(2);
    let file_path = store_path.join("objects").join(subdir).join(filename);
    let file_content = read(file_path)
        .map_err(|e| format!("Failed to read file: {e}"))?;
    OObject::deserialize(&file_content)
}

pub fn write_object(store_path: &Path, obj: &OObject) -> Result<OObjectId, String> {
    /* Algoritmo */
    // 1. Serializza l'oggetto → Vec<u8>
    // 2. Calcola hash dei byte serializzati → [u8; 32]
    // 3. Converti hash in hex string → String
    // 4. Costruisci path: .ogit/objects/aa/bbccdd... (primi 2 char = subdirectory)
    // 5. Crea subdirectory se non esiste
    // 6. Scrivi file (se non esiste già)
    // 7. Restituisci OObjectId
    
    let ser = obj.serialize();
    let hashed_ser = hash_bytes(&ser);
    let hashed_hexed = bytes_to_hex(&hashed_ser);

    let (subdir, filename) = hashed_hexed.split_at(2);
    let dir_path = store_path.join("objects").join(subdir);
    let file_path = dir_path.join(filename);

    create_dir_all(&dir_path)
        .map_err(|e| format!("Failed to create dir: {e}"))?;

    // Evita scritture inutili su BLOB grandi
    if !file_path.exists() {
    write(&file_path, &ser)
        .map_err(|e| format!("Failed to write: {e}"))?;
    }
    
    Ok(OObjectId(hashed_hexed))
}

pub fn create_commit(
    store_path: &Path,
    tree: &OObjectId,
    parent: Option<&OObjectId>,
    author: &str,
    message: &str,
) -> Result<OObjectId, String> {
    /* Algoritmo: */
    // 1. Costruisci Commit struct
    // 2. Serializza
    // 3. Crea OObject::new_commit(payload)
    // 4. Salva con write_object
    // 5. Restituisci hash
    
    let data = Commit { tree: tree.clone(), parent: parent.cloned(), author: author.to_string(), message: message.to_string() };
    let commit = OObject::new_commit(data.serialize());
    write_object(store_path, &commit)
}

