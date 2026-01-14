use sha2::{Digest, Sha256};

pub fn hash_bytes(bytes: &[u8]) -> [u8; 32] {
    // create a Sha256 object
    let mut hasher = Sha256::new();

    hasher.update(&bytes);

    let result = hasher.finalize();

    result.into()
}