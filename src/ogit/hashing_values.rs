use sha2::{Digest, Sha256};
use std::fmt::Write;

#[must_use]
pub fn hash_bytes(bytes: &[u8]) -> [u8; 32] {
    // create a Sha256 object
    let mut hasher = Sha256::new();

    hasher.update(bytes);

    let result = hasher.finalize();

    result.into()
}

#[must_use]
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    // Per ogni byte in bytes:
    //   - formatta come 2 caratteri hex (usa "{:02x}")
    //   - raccogli tutto in una String
    
    //THIS OLD VERSION WAS INEFFICIENT DUE TO many heap allocations and repeated reallocation/growth of the final buffer.
    //bytes.iter().map(|b| format!("{b:02x}")).collect::<String>()

    // This version: Uses single allocation (pre-sized with with_capacity),
    //  no intermediate Strings and
    //  appends directly into the final buffer via write!
    bytes.iter().fold(String::with_capacity(bytes.len() * 2), |mut acc, b| {
        write!(&mut acc, "{b:02x}").unwrap();
        acc
    })
}