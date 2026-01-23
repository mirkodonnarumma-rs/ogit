use sha2::{Digest, Sha256};

pub fn hash_bytes(bytes: &[u8]) -> [u8; 32] {
    // create a Sha256 object
    let mut hasher = Sha256::new();

    hasher.update(bytes);

    let result = hasher.finalize();

    result.into()
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    // Per ogni byte in bytes:
    //   - formatta come 2 caratteri hex (usa "{:02x}")
    //   - raccogli tutto in una String
    bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>()
}