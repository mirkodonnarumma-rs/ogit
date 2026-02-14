#[cfg(test)]
mod tests {
    use ogit::object::{Commit, OObjectId};

        #[test]
    fn test_commit_serialize_deserialize_with_parent() {
        let original = Commit {
            tree: OObjectId("abc123".to_string()),
            parent: Some(OObjectId("def456".to_string())),
            author: "Test Author".to_string(),
            message: "Initial commit".to_string(),
        };
        
        let serialized = original.serialize();
        let deserialized = Commit::deserialize(&serialized).unwrap();
        
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_commit_serialize_deserialize_without_parent() {
        let original = Commit {
            tree: OObjectId("abc123".to_string()),
            parent: None,
            author: "Test Author".to_string(),
            message: "First commit".to_string(),
        };
        
        let serialized = original.serialize();
        let deserialized = Commit::deserialize(&serialized).unwrap();
        
        assert_eq!(original, deserialized);
    }
}