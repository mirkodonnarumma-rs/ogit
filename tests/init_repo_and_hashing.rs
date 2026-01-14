//! tests/step1_verification.rs
//! Test di integrazione Step 1: init_repo + hash_bytes
//! Completamente indipendenti da main.rs

use std::path::Path;
use std::fs;
use std::sync::Mutex;

use ogit::hashing_values::hash_bytes;
use ogit::initialize_repository::init_repo;

/// Mutex globale per sincronizzare i test del filesystem
/// Previene race condition quando più test usano .ogit/ simultaneamente
static FS_LOCK: Mutex<()> = Mutex::new(());

/// Helper per pulire e proteggere accesso a .ogit/
fn with_clean_repo<F>(test_fn: F)
where
    F: FnOnce(),
{
    let _guard = FS_LOCK.lock().unwrap();
    let _ = fs::remove_dir_all(".ogit");
    test_fn();
    let _ = fs::remove_dir_all(".ogit");
}

/// Test A1: init_repo crea directory .ogit
#[test]
fn test_init_repo_creates_dotogit() {
    with_clean_repo(|| {
        let result = init_repo();
        assert!(result.is_ok(), "init_repo dovrebbe riuscire");

        assert!(
            Path::new(".ogit").exists(),
            ".ogit directory non creata"
        );
    });
}

/// Test A2: init_repo crea directory objects
#[test]
fn test_init_repo_creates_objects_dir() {
    with_clean_repo(|| {
        init_repo().expect("init_repo fallito");
        assert!(
            Path::new(".ogit/objects").exists(),
            ".ogit/objects directory non creata"
        );
    });
}

/// Test A3: init_repo crea directory refs/heads
#[test]
fn test_init_repo_creates_refs_heads() {
    with_clean_repo(|| {
        init_repo().expect("init_repo fallito");
        assert!(
            Path::new(".ogit/refs/heads").exists(),
            ".ogit/refs/heads directory non creata"
        );
    });
}

/// Test A4: init_repo è idempotente (accetta se esiste)
#[test]
fn test_init_repo_is_idempotent() {
    with_clean_repo(|| {
        // Prima inizializzazione
        init_repo().expect("primo init_repo fallito");

        // Seconda inizializzazione (non dovrebbe fallire)
        let result = init_repo();
        assert!(result.is_ok(), "init_repo dovrebbe essere idempotente");
    });
}

/// Test B1: hash_bytes è deterministico
#[test]
fn test_hash_bytes_is_deterministic() {
    let input = b"hello";
    let hash1 = hash_bytes(input);
    let hash2 = hash_bytes(input);
    assert_eq!(hash1, hash2, "hash dovrebbe essere deterministico");
}

/// Test B2: hash_bytes produce risultati diversi per input diversi
#[test]
fn test_hash_bytes_differs_for_different_input() {
    let hash1 = hash_bytes(b"hello");
    let hash2 = hash_bytes(b"hello ");
    assert_ne!(hash1, hash2, "input diversi dovrebbero produrre hash diversi");
}

/// Test B3: hash_bytes funziona con bytes vuoti
#[test]
fn test_hash_bytes_with_empty_bytes() {
    let hash = hash_bytes(b"");
    // Array di 32 byte, dovrebbe essere non-zero per hash SHA256
    let zero_array = [0u8; 32];
    assert_ne!(hash, zero_array, "hash di vuoto non dovrebbe essere zero");
}

/// Test B4: hash_bytes funziona con slice statico
#[test]
fn test_hash_bytes_with_static_slice() {
    let hash = hash_bytes(&[1u8, 2, 3, 4, 5]);
    // Verifica che l'output sia un array valido (sempre 32 byte per SHA256)
    assert_eq!(hash.len(), 32, "SHA256 dovrebbe sempre produrre 32 byte");
}

/// Test B5: hash_bytes funziona con Vec temporaneo
#[test]
fn test_hash_bytes_with_temporary_vec() {
    let temp_vec = vec![1u8, 2, 3, 4, 5];
    let hash = hash_bytes(&temp_vec);
    assert_eq!(hash.len(), 32, "SHA256 dovrebbe sempre produrre 32 byte");
    // temp_vec viene droppato qui
}

/// Test C1: hash_bytes indipendente dalla fonte (Vec vs slice)
#[test]
fn test_hash_bytes_independent_of_source() {
    let temp_buffer = vec![1u8, 2, 3, 4, 5];
    let hash_from_temp = hash_bytes(&temp_buffer);
    // temp_buffer droppato qui

    let hash_from_static = hash_bytes(&[1u8, 2, 3, 4, 5]);

    assert_eq!(
        hash_from_temp, hash_from_static,
        "hash dovrebbe essere indipendente dalla fonte (Vec vs slice)"
    );
}

/// Test C2: hash_bytes restituisce array owned
#[test]
fn test_hash_bytes_returns_owned_array() {
    let hash = hash_bytes(b"test");
    let hash_copy = hash.clone();
    assert_eq!(hash, hash_copy, "array clonato dovrebbe essere identico");
}

/// Test C3: hash_bytes produce array di 32 byte (SHA256)
#[test]
fn test_hash_bytes_produces_32_bytes() {
    let inputs: Vec<&[u8]> = vec![b"", b"a", b"hello", b"the quick brown fox"];
    
    for input in inputs {
        let hash = hash_bytes(input);
        assert_eq!(
            hash.len(),
            32,
            "SHA256 dovrebbe sempre produrre 32 byte, non {}",
            hash.len()
        );
    }
}