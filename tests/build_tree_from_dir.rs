#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;

    use ogit::object::{OObjectType, TreeEntry};
    use ogit::store::read_object;
    use ogit::tree::build_tree_from_dir;

    fn setup_test_dir(name: &str) -> std::path::PathBuf {
        let dir = env::temp_dir().join(format!("ogit_tree_test_{}_{}", name, std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::create_dir_all(dir.join(".ogit/objects")).unwrap();
        dir
    }

    fn cleanup(dir: &Path) {
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_build_tree_single_file() {
        let test_dir = setup_test_dir("single");
        let content_dir = test_dir.join("content");
        fs::create_dir_all(&content_dir).unwrap();
        
        // Crea file
        let mut f = File::create(content_dir.join("hello.txt")).unwrap();
        f.write_all(b"Hello, world!").unwrap();
        
        let store_path = test_dir.join(".ogit");
        let tree_id = build_tree_from_dir(&store_path, &content_dir).unwrap();
        
        // Verifica che tree esista
        let (subdir, filename) = tree_id.as_str().split_at(2);
        let tree_path = store_path.join("objects").join(subdir).join(filename);
        assert!(tree_path.exists());
        
        cleanup(&test_dir);
    }

    #[test]
    fn test_build_tree_with_subdir() {
        let test_dir = setup_test_dir("subdir");
        let content_dir = test_dir.join("content");
        fs::create_dir_all(&content_dir).unwrap();
        fs::create_dir_all(content_dir.join("src")).unwrap();
        
        // File nella root
        File::create(content_dir.join("readme.txt")).unwrap()
            .write_all(b"README").unwrap();
        
        // File nella subdir
        File::create(content_dir.join("src").join("main.rs")).unwrap()
            .write_all(b"fn main() {}").unwrap();
        
        let store_path = test_dir.join(".ogit");
        let tree_id = build_tree_from_dir(&store_path, &content_dir).unwrap();
        
        // Leggi tree e verifica entries
        let tree_obj = read_object(&store_path, &tree_id).unwrap();
        let entries = TreeEntry::deserialize_tree(&tree_obj.data).unwrap();
        
        assert_eq!(entries.len(), 2);
        // Ordinati alfabeticamente
        assert_eq!(entries[0].name, "readme.txt");
        assert_eq!(entries[0].kind, OObjectType::Blob);
        assert_eq!(entries[1].name, "src");
        assert_eq!(entries[1].kind, OObjectType::Tree);
        
        cleanup(&test_dir);
    }

    #[test]
    fn test_build_tree_ignores_ogit() {
        let test_dir = setup_test_dir("ignore");
        let content_dir = test_dir.join("content");
        fs::create_dir_all(&content_dir).unwrap();
        fs::create_dir_all(content_dir.join(".ogit")).unwrap();
        
        File::create(content_dir.join("file.txt")).unwrap()
            .write_all(b"data").unwrap();
        File::create(content_dir.join(".ogit").join("secret")).unwrap()
            .write_all(b"ignored").unwrap();
        
        let store_path = test_dir.join(".ogit");
        let tree_id = build_tree_from_dir(&store_path, &content_dir).unwrap();
        
        let tree_obj = read_object(&store_path, &tree_id).unwrap();
        let entries = TreeEntry::deserialize_tree(&tree_obj.data).unwrap();
        
        // Solo file.txt, .ogit ignorato
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "file.txt");
        
        cleanup(&test_dir);
    }
}