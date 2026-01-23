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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OObjectId(pub String);

impl OObjectId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}