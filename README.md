![OGit](assets/ogit.png)

# ogit — Minimal Git-like Object Store (Rust)

`ogit` è un mini progetto CLI scritto in Rust che implementa un object store
content-addressed ispirato a Git.

Lo scopo principale del progetto è **didattico**: esplorare in modo pratico
ownership, borrowing e lifetimes attraverso un dominio reale (filesystem + hashing),
evitando strutture artificiali.

## Stato del progetto

Questo repository è sviluppato incrementalmente.

**STEP 1 completato:**

- inizializzazione repository (`ogit init`)
- helper di hashing deterministico su byte slice (`hash_bytes`)

**STEP 2 completato:**

- scrittura blob object su disco (`write_object`)
- lettura blob object da disco (`read_object`)
- comandi CLI `ogit store <file>` e `ogit cat <hash>`

**STEP 3 completato:**

- struttura `TreeEntry` per rappresentare entry di directory
- serializzazione/deserializzazione tree con ordinamento alfabetico
- funzione ricorsiva `build_tree_from_dir` per traversal directory
- comando CLI `ogit write-tree <dir>`

**STEP 4 completato:**

- struttura `Commit` con tree, parent opzionale, author e message
- serializzazione/deserializzazione commit
- funzione `create_commit` per creazione e persistenza
- comando CLI `ogit commit -m "message"` con tracking HEAD

## Requisiti

- Rust stable
- Nessuna dipendenza obbligatoria oltre `std`
- CLI implementata manualmente (senza `clap` nelle prime fasi)

## Scelte di design (STEP 1)

- Le operazioni di filesystem sono incapsulate nel core (`ogit`), non nel `main`.
- I path sono rappresentati tramite `Path`, evitando stringhe raw.
- L'helper `hash_bytes`:
  - accetta `&[u8]` (borrow puro)
  - restituisce un valore posseduto rappresentato da un array fisso di 32 elementi di tipo u8
  - non espone lifetime esplicite

Queste scelte sono intenzionali per evitare dangling references e coupling prematuro.

## Scelte di design (STEP 2)

- Il formato on-disk è compatibile con Git: `<type> <size>\0<data>` (es. `blob 13\0Hello, world!`).
- La directory sharding segue lo schema Git: i primi 2 caratteri hex dell'hash diventano la subdirectory, i restanti il filename (`.ogit/objects/ab/cd1234...`).
- Le scritture sono idempotenti: se il file esiste già non viene riscritto, evitando I/O inutile su blob grandi.
- `OObjectId` è un newtype su `String` che previene confusione con stringhe generiche.
- La conversione hex è ottimizzata con una singola allocazione tramite `fold` + `String::with_capacity`.
- Nessun lifetime esplicito nell'API pubblica: gli input sono borrowed (`&Path`, `&OObject`, `&OObjectId`), i valori di ritorno sono sempre owned (`OObject`, `OObjectId`).
- La deserializzazione valida il formato (header, separatore null, corrispondenza size/data) e restituisce errori descrittivi.
- I comandi CLI `store` e `cat` delegano tutta la logica al core (`ogit`), mantenendo `main.rs` come thin layer.

## Scelte di design (STEP 3)

- Il formato tree è testuale: `<type> <hash> <name>\n` per ogni entry.
- Le entry sono ordinate alfabeticamente per nome prima della serializzazione.
- Il parsing usa `splitn(3, ' ')` per supportare filename con spazi.
- La directory `.ogit` viene ignorata durante il traversal ricorsivo.
- La ricorsione separa chiaramente `store_path` (dove salvare oggetti) da `dir_path` (cosa processare).

## Scelte di design (STEP 4)

- Il formato commit è testuale con campi prefissati: `tree`, `parent` (opzionale), `author`, `message`.
- Il campo `parent` è `Option<OObjectId>`: `None` per il primo commit, `Some(hash)` per i successivi.
- Il file `.ogit/HEAD` memorizza l'hash del commit corrente come reference semplice.
- `create_commit` è una funzione pura che costruisce, serializza e persiste il commit atomicamente.

## Roadmap

- [x] STEP 1 — init repository + hashing
- [x] STEP 2 — blob object (read/write)
- [x] STEP 3 — tree object
- [x] STEP 4 — commit object
- [ ] STEP 5 — CLI polish