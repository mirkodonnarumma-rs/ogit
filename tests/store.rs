#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;

    use ogit::object::OObject;
    use ogit::store::read_object;
    use ogit::store::write_object;

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