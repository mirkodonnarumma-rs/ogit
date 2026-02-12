//!ogit <command> [args...]
//! Commands:
//!   init              Inizializza repository
//!   store <file>      Salva file come blob, stampa hash
//!   cat <hash>        Mostra contenuto oggetto

use std::env;
use std::fs::read;
use std::path::Path;
use std::process;

use ogit::initialize_repository::init_repo;
use ogit::object::{OObject, OObjectId};
use ogit::store::{read_object, write_object};
use ogit::tree::build_tree_from_dir;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // args[0] = "ogit" (nome programma)
    // args[1] = comando
    // args[2..] = argomenti del comando
    
    if args.len() < 2 {
        eprintln!("Usage: ogit <command> [args]");
        eprintln!("Commands: init, store, cat");
        process::exit(1);
    }
    
    let command = args[1].as_str();
    
    let result = match command {
        "init" => cmd_init(),
        "store" => cmd_store(&args[2..]),
        "cat" => cmd_cat(&args[2..]),
        "write-tree" => cmd_write_tree(&args[2..]),
        _ => {
            eprintln!("Unknown command: {}", command);
            process::exit(1);
        }
    };
    
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn cmd_init() -> Result<(), String> {
    init_repo().map_err(|e| e.to_string())
}

fn cmd_store(args: &[String]) -> Result<(), String> {
    /* Algoritmo */
    // 1. Verifica che args contenga almeno 1 elemento (il path del file)
    // 2. Leggi il file come Vec<u8>
    // 3. Crea OObject::new_blob(...)
    // 4. Chiama write_object(...)
    // 5. Stampa l'hash
    if args.is_empty() {
        return Err("Usage: ogit store <file>".into());
    }

    let file_path = Path::new(&args[0]);
    let file_content = read(file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let obj = OObject::new_blob(file_content);
    
    let store_path = Path::new(".ogit");
    let id = write_object(store_path, &obj)?;

    println!("{}", id.as_str());

    Ok(())
}

fn cmd_cat(args: &[String]) -> Result<(), String> {
    /* Algoritmo */
    // 1. Verifica args[0] esista (l'hash)
    // 2. Crea OObjectId dall'hash
    // 3. Chiama read_object
    // 4. Stampa il contenuto (obj.data come UTF-8 o hex)
     if args.is_empty() {
        return Err("Usage: ogit cat <hash>".into());
    }
    
    let hash = &args[0];
    let id = OObjectId(hash.clone());
    
    let store_path = Path::new(".ogit");
    let obj = read_object(store_path, &id)?;

    // Prova a stampare come UTF-8, altrimenti mostra hex
    match std::str::from_utf8(&obj.data) {
        Ok(text) => println!("{}", text),
        Err(_) => println!("{:?}", obj.data),
    }
    
    Ok(())
}

fn cmd_write_tree(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("Usage: ogit write-tree <dir>".into());
    }
    
    let dir_path = Path::new(&args[0]);
    let store_path = Path::new(".ogit");
    
    let id = build_tree_from_dir(store_path, dir_path)?;
    println!("{}", id.as_str());
    
    Ok(())
}