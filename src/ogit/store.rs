use std::path::Path;
use std::fs::{create_dir_all, read, write};

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
    let data = OObject::deserialize(&file_content);
    data
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn test_write_then_read_object() {
        let temp_dir = env::temp_dir().join(format!("ogit_test_rw_{}", std::process::id()));
        fs::create_dir_all(&temp_dir).unwrap();

        let original = OObject::new_blob(b"roundtrip test".to_vec());
        let id = write_object(&temp_dir, &original).unwrap();
        let loaded = read_object(&temp_dir, &id).unwrap();

        assert_eq!(original, loaded);

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_write_object_creates_file() {
        // 1. Setup: directory temporanea unica
        let temp_dir = env::temp_dir().join(format!("ogit_test_{}", std::process::id()));
        fs::create_dir_all(&temp_dir).unwrap();

        // 2. Crea e scrivi oggetto
        let obj = OObject::new_blob(b"test content".to_vec());
        let id = write_object(&temp_dir, &obj).unwrap();

        // 3. Verifica che il file esista
        let (subdir, filename) = id.as_str().split_at(2);
        let file_path = temp_dir.join("objects").join(subdir).join(filename);
        assert!(file_path.exists(), "Object file should exist");

        // 4. Verifica contenuto
        let stored_bytes = fs::read(&file_path).unwrap();
        assert_eq!(stored_bytes, obj.serialize());

        // 5. Cleanup
        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_write_object_is_idempotent() {
        let temp_dir = env::temp_dir().join(format!("ogit_test_idem_{}", std::process::id()));
        fs::create_dir_all(&temp_dir).unwrap();

        let obj = OObject::new_blob(b"same content".to_vec());
        
        let id1 = write_object(&temp_dir, &obj).unwrap();
        let id2 = write_object(&temp_dir, &obj).unwrap();

        // Stesso contenuto → stesso hash
        assert_eq!(id1, id2);

        fs::remove_dir_all(&temp_dir).unwrap();
    }
}