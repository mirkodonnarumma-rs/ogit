//!OObjectType   — enum con varianti Blob, Tree, Commit
//!OObjectId     — newtype su String (hex hash)
//!OObject       — struct con kind: OObjectType, data: Vec<u8>

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OObjectType {
    Blob,
    Tree,
    Commit,
}

impl OObjectType {
    pub fn as_str(&self) -> &'static str {
        match self {
            OObjectType::Blob => "blob",
            OObjectType::Tree => "tree",
            OObjectType::Commit => "commit",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct OObject {
    pub kind: OObjectType,
    pub data: Vec<u8>,
}

impl OObject {
    pub fn new_blob(data: Vec<u8>) -> Self {
        OObject { kind: OObjectType::Blob, data }
    }

    pub fn new_tree(data: Vec<u8>) -> Self {
        OObject { kind: OObjectType::Tree, data }
    }

    pub fn new_commit(data: Vec<u8>) -> Self {
        OObject { kind: OObjectType::Commit, data }
    }
    pub fn serialize(&self) -> Vec<u8> {
        let header = format!("{} {}\0", self.kind.as_str(), self.data.len());
        let mut result = header.into_bytes();
        result.extend_from_slice(&self.data);
        result
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, String> {
        // 1. Trova posizione di \0
        let null_pos = bytes
            .iter()
            .position(|&b| b == 0)
            .ok_or("Missing null byte")?;
        
        // 2. Splitta header e data
        let header_bytes = &bytes[..null_pos];
        let data = &bytes[null_pos + 1..];
        
        // 3. Parsa header come UTF-8
        let header = std::str::from_utf8(header_bytes)
            .map_err(|_| "Invalid UTF-8 in header")?;
        
        // 4. Splitta su spazio: "blob 5" → ["blob", "5"]
        let parts: Vec<&str> = header.split(' ').collect();
        if parts.len() != 2 {
            return Err("Invalid header format".into());
        }
        
        // 5. Parsa tipo e size
        let kind = match parts[0] {
            "blob" => OObjectType::Blob,
            "tree" => OObjectType::Tree,
            "commit" => OObjectType::Commit,
            _ => return Err("Unknown object type".into()),
        };
        
        let size: usize = parts[1]
            .parse()
            .map_err(|_| "Invalid size")?;
        
        // 6. Valida size
        if data.len() != size {
            return Err("Size mismatch".into());
        }
        
        // 7. Costruisci OObject
        Ok(OObject {
            kind,
            data: data.to_vec(),  // &[u8] → Vec<u8>
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OObjectId(pub String);

impl OObjectId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}