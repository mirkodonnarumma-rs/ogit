//!`OObjectType`   — enum con varianti Blob, Tree, Commit
//!`OObjectId`     — newtype su String (hex hash)
//!`OObject`       — struct con kind: `OObjectType`, data: Vec<u8>

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeEntry {
    pub kind: OObjectType,    // Blob o Tree
    pub hash: OObjectId,      // hash dell'oggetto
    pub name: String,         // nome file/directory
}

impl TreeEntry {
    pub fn new(kind: OObjectType, hash: OObjectId, name: String) -> Self {
        Self { kind, hash, name }
    }
    pub fn to_line(&self) -> String {
        // formato: "<type> <hash> <name>"
        format!("{} {} {}", self.kind.as_str(), self.hash.as_str(), self.name)
    }
    pub fn serialize_tree(entries: &[TreeEntry]) -> Vec<u8> {
        // 1. Ordina entries per nome (alfabetico)
        // 2. Converti ogni entry in linea
        // 3. Unisci con \n
        // 4. Converti in bytes
        let mut sorted = entries.to_vec();
        sorted.sort_by(|a, b| a.name.cmp(&b.name));
        sorted.iter()
            .map(|e| e.to_line())
            .collect::<Vec<_>>()
            .join("\n")
            .into_bytes()
    }
    /// **Algoritmo per parsare una singola linea:**
    ///        "blob a1b2c3... readme.txt"
    ///        │     │         │
    ///        │     │         └── name (tutto dopo secondo spazio)
    ///        │     └── hash (secondo token)
    ///        └── type (primo token) 
    pub fn deserialize_tree(bytes: &[u8]) -> Result<Vec<TreeEntry>, String> {
        // 1. Converti bytes in stringa UTF-8
        // 2. Splitta per \n
        // 3. Per ogni linea, parsa in TreeEntry
        // 4. Raccogli in Vec
        let content = std::str::from_utf8(bytes)
            .map_err(|_| "Invalid UTF-8 in tree")?;
        
        let mut entries = Vec::new();
        
        for line in content.lines() {
            if line.is_empty() {
                continue;
            }
            
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
            if parts.len() != 3 {
                return Err(format!("Invalid tree entry: {}", line));
            }
            
            let kind = match parts[0] {
                "blob" => OObjectType::Blob,
                "tree" => OObjectType::Tree,
                _ => return Err(format!("Unknown type: {}", parts[0])),
            };
            
            let hash = OObjectId(parts[1].to_string());
            let name = parts[2].to_string();
            
            entries.push(TreeEntry { kind, hash, name });
    }
    
    Ok(entries)
}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OObjectType {
    Blob,
    Tree,
    Commit,
}

impl OObjectType {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Blob => "blob",
            Self::Tree => "tree",
            Self::Commit => "commit",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
//<type> <size>\0<data>
// example blob 5\0Hello
pub struct OObject {
    pub kind: OObjectType,
    pub data: Vec<u8>,
}

impl OObject {
    #[must_use]
    pub const fn new_blob(data: Vec<u8>) -> Self {
        Self { kind: OObjectType::Blob, data }
    }

    #[must_use]
    pub const fn new_tree(data: Vec<u8>) -> Self {
        Self { kind: OObjectType::Tree, data }
    }

    #[must_use]
    pub const fn new_commit(data: Vec<u8>) -> Self {
        Self { kind: OObjectType::Commit, data }
    }
    #[must_use]
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
        Ok(Self {
            kind,
            data: data.to_vec(),  // &[u8] → Vec<u8>
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OObjectId(pub String);

impl OObjectId {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Formato payload
/// ```text
/// tree <tree_hash>
/// parent <parent_hash>    ← opzionale
/// author <name>
/// message <text>
/// ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commit {
    pub tree: OObjectId,
    pub parent: Option<OObjectId>,
    pub author: String,
    pub message: String,
}

impl Commit {
    pub fn serialize(&self) -> Vec<u8> {
        let mut lines = Vec::new();
        
        lines.push(format!("tree {}", self.tree.as_str()));
        
        if let Some(ref parent) = self.parent {
            lines.push(format!("parent {}", parent.as_str()));
        }
        
        lines.push(format!("author {}", self.author));
        lines.push(format!("message {}", self.message));
        
        lines.join("\n").into_bytes()
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, String> {
        let content = std::str::from_utf8(bytes)
            .map_err(|_| "Invalid UTF-8 in commit")?;
        
        let mut tree: Option<OObjectId> = None;
        let mut parent: Option<OObjectId> = None;
        let mut author: Option<String> = None;
        let mut message: Option<String> = None;
        
        for line in content.lines() {
            if let Some(hash) = line.strip_prefix("tree ") {
                tree = Some(OObjectId(hash.to_string()));
            } else if let Some(hash) = line.strip_prefix("parent ") {
                parent = Some(OObjectId(hash.to_string()));
            } else if let Some(name) = line.strip_prefix("author ") {
                author = Some(name.to_string());
            } else if let Some(msg) = line.strip_prefix("message ") {
                message = Some(msg.to_string());
            }
        }
        
        Ok(Commit {
            tree: tree.ok_or("Missing tree")?,
            parent,
            author: author.ok_or("Missing author")?,
            message: message.ok_or("Missing message")?,
        })
    }
}