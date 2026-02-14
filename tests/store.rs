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

mod create_commit_tests {
    use std::env;
    use std::fs;

    use ogit::object::Commit;
    use ogit::object::OObjectId;
    use ogit::store::create_commit;
    use ogit::store::read_object;

    fn setup_test_dir(name: &str) -> std::path::PathBuf {
        let dir = env::temp_dir().join(format!("ogit_commit_test_{}_{}", name, std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("objects")).unwrap();
        dir
    }

    #[test]
    fn test_create_commit_without_parent() {
        let store = setup_test_dir("no_parent");
        let tree_id = OObjectId("abc123def456".to_string());
        
        let commit_id = create_commit(&store, &tree_id, None, "Author", "First commit").unwrap();
        
        let obj = read_object(&store, &commit_id).unwrap();
        let commit = Commit::deserialize(&obj.data).unwrap();
        
        assert_eq!(commit.tree.as_str(), "abc123def456");
        assert!(commit.parent.is_none());
        assert_eq!(commit.author, "Author");
        assert_eq!(commit.message, "First commit");
        
        fs::remove_dir_all(&store).unwrap();
    }

    #[test]
    fn test_create_commit_with_parent() {
        let store = setup_test_dir("with_parent");
        let tree_id = OObjectId("abc123".to_string());
        let parent_id = OObjectId("parent789".to_string());
        
        let commit_id = create_commit(&store, &tree_id, Some(&parent_id), "Author", "Second commit").unwrap();
        
        let obj = read_object(&store, &commit_id).unwrap();
        let commit = Commit::deserialize(&obj.data).unwrap();
        
        assert_eq!(commit.parent.unwrap().as_str(), "parent789");
        
        fs::remove_dir_all(&store).unwrap();
    }
}