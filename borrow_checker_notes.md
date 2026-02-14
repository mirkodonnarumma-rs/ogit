# Borrow Checker Notes

Errori reali incontrati durante lo sviluppo di ogit e relative soluzioni.

---

## 1. `extend()` vs `extend_from_slice()`

**Contesto:** `OObject::serialize()`

**Errore:**

```rust
result.extend(&self.data);
// cannot move out of `self.data` which is behind a shared reference
```

**Causa:** `extend()` su `&Vec<u8>` può creare ambiguità di ownership.

**Soluzione:**

```rust
result.extend_from_slice(&self.data);
```

**Lezione:** `extend_from_slice` è più esplicito e idiomatico per copiare slice di byte.

---

## 2. Confusione tra file path e store path

**Contesto:** `write_object()`

**Errore:**

```rust
let hash = write_object(&path, &blob);  // path è il file da leggere
```

**Causa:** `write_object` richiede `store_path` (dove salvare), non il path del file sorgente.

**Soluzione:**

```rust
let store_path = Path::new(".ogit");
let hash = write_object(store_path, &blob);
```

**Lezione:** nominare chiaramente le variabili (`file_path` vs `store_path`) previene errori logici.

---

## 3. `create_dir_all` sul path sbagliato

**Contesto:** `write_object()`

**Errore:**

```rust
create_dir_all(&file_path);  // crea directory dove dovrebbe essere il file
// Error: "Is a directory"
```

**Causa:** `file_path` include il filename, ma `create_dir_all` crea directory.

**Soluzione:**

```rust
let dir_path = store_path.join("objects").join(subdir);
let file_path = dir_path.join(filename);
create_dir_all(&dir_path)?;  // crea solo la directory
write(&file_path, &content)?;
```

**Lezione:** separare chiaramente `dir_path` e `file_path` nelle variabili.

---

## 4. Argomenti invertiti in funzione ricorsiva

**Contesto:** `build_tree_from_dir()`

**Errore:**

```rust
let hash = build_tree_from_dir(&path, &dir_path);  // invertiti
```

**Causa:** firma è `(store_path, dir_path)`, ma passati al contrario.

**Soluzione:**

```rust
let hash = build_tree_from_dir(store_path, &path)?;
```

**Lezione:** mantenere ordine consistente dei parametri (store sempre primo).

---

## Pattern generali appresi

1. **Owned vs Borrowed return:** le funzioni I/O restituiscono sempre dati owned (`Vec<u8>`, `String`) perché i buffer di lettura sono temporanei.

2. **Lifetime impliciti:** evitare lifetime espliciti nell'API pubblica semplifica l'uso e previene errori.

3. **Newtype pattern:** `OObjectId(String)` previene confusione con stringhe generiche a compile-time.

4. **Separazione responsabilità:** `store_path` (dove salvare) vs `file_path`/`dir_path` (cosa processare) devono essere sempre distinti.
