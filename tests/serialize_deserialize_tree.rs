#[cfg(test)]
mod tests {
    use ogit::object::{OObjectId, OObjectType, TreeEntry};

    

    #[test]
    fn test_tree_entry_to_line() {
        let entry = TreeEntry {
            kind: OObjectType::Blob,
            hash: OObjectId("abc123".to_string()),
            name: "readme.txt".to_string(),
        };
        
        assert_eq!(entry.to_line(), "blob abc123 readme.txt");
    }

    #[test]
    fn test_serialize_deserialize_tree_roundtrip() {
        let entries = vec![
            TreeEntry {
                kind: OObjectType::Blob,
                hash: OObjectId("aaa111".to_string()),
                name: "zebra.txt".to_string(),
            },
            TreeEntry {
                kind: OObjectType::Tree,
                hash: OObjectId("bbb222".to_string()),
                name: "alpha".to_string(),
            },
            TreeEntry {
                kind: OObjectType::Blob,
                hash: OObjectId("ccc333".to_string()),
                name: "middle.rs".to_string(),
            },
        ];
        
        let serialized = TreeEntry::serialize_tree(&entries);
        let deserialized = TreeEntry::deserialize_tree(&serialized).unwrap();
        
        // Verifica ordinamento alfabetico per nome
        assert_eq!(deserialized.len(), 3);
        assert_eq!(deserialized[0].name, "alpha");
        assert_eq!(deserialized[1].name, "middle.rs");
        assert_eq!(deserialized[2].name, "zebra.txt");
        
        // Verifica contenuto completo
        assert_eq!(deserialized[0].kind, OObjectType::Tree);
        assert_eq!(deserialized[0].hash.as_str(), "bbb222");
    }

    #[test]
    fn test_tree_with_spaces_in_filename() {
        let entries = vec![
            TreeEntry {
                kind: OObjectType::Blob,
                hash: OObjectId("abc123".to_string()),
                name: "my file with spaces.txt".to_string(),
            },
        ];
        
        let serialized = TreeEntry::serialize_tree(&entries);
        let deserialized = TreeEntry::deserialize_tree(&serialized).unwrap();
        
        assert_eq!(deserialized[0].name, "my file with spaces.txt");
    }
}