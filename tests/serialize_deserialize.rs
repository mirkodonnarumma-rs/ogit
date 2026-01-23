#[cfg(test)]
mod tests {
    use ogit::object::OObject;

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let original = OObject::new_blob(b"Hello, world!".to_vec());
        let serialized = original.serialize();
        let deserialized = OObject::deserialize(&serialized).unwrap();
        
        assert_eq!(original, deserialized);
    }
}