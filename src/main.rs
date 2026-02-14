//!ogit <command> [args...]
//! Commands:
//!   init              Inizializza repository
//!   store <file>      Salva file come blob, stampa hash
//!   cat <hash>        Mostra contenuto oggetto

use std::env;
use std::fs::read;
use std::path::Path;
use std::process;
use std::str::from_utf8;

use ogit::initialize_repository::init_repo;
use ogit::object::{Commit, OObject, OObjectId, OObjectType, TreeEntry};
use ogit::store::{read_object, write_object, create_commit};
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
        "commit" => cmd_commit(&args[2..]),
        "show" => cmd_show(&args[2..]),
        "ls-objects" => cmd_ls_objects(&args[2..]),
        "log" => cmd_log(&args[2..]),
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
    match from_utf8(&obj.data) {
        Ok(text) => println!("{}", text),
        Err(_) => println!("{:?}", obj.data),
    }
    
    Ok(())
}

/// Differenza da cat: show mostra anche il tipo dell'oggetto e formatta meglio l'output.
fn cmd_show(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("Usage: ogit show <hash>".into());
    }
    
    let id = OObjectId(args[0].clone());
    let store_path = Path::new(".ogit");
    let obj = read_object(store_path, &id)?;
    
    println!("type: {}", obj.kind.as_str());
    println!("size: {}", obj.data.len());
    println!("---");
    
    match obj.kind {
        OObjectType::Blob => {
            match from_utf8(&obj.data) {
                Ok(text) => println!("{}", text),
                Err(_) => println!("[binary data, {} bytes]", obj.data.len()),
            }
        }
        OObjectType::Tree => {
            let entries = TreeEntry::deserialize_tree(&obj.data)?;
            for entry in entries {
                println!("{} {} {}", entry.kind.as_str(), entry.hash.as_str(), entry.name);
            }
        }
        OObjectType::Commit => {
            let commit = Commit::deserialize(&obj.data)?;
            println!("tree:    {}", commit.tree.as_str());
            if let Some(parent) = &commit.parent {
                println!("parent:  {}", parent.as_str());
            }
            println!("author:  {}", commit.author);
            println!("message: {}", commit.message);
        }
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

fn cmd_commit(args: &[String]) -> Result<(), String> {
    // Parsing: -m "message"
    if args.len() < 2 || args[0] != "-m" {
        return Err("Usage: ogit commit -m \"message\"".into());
    }
    let message = &args[1];
    
    let store_path = Path::new(".ogit");
    
    // 1. Costruisci tree dalla directory corrente
    let tree_id = build_tree_from_dir(store_path, Path::new("."))?;
    
    // 2. Leggi parent da HEAD (se esiste e contiene hash valido)
    let head_path = store_path.join("HEAD");
    let parent = if head_path.exists() {
        let content = std::fs::read_to_string(&head_path)
            .map_err(|e| format!("Failed to read HEAD: {}", e))?;
        let trimmed = content.trim();
        if trimmed.len() == 64 && trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
            Some(OObjectId(trimmed.to_string()))
        } else {
            None
        }
    } else {
        None
    };
    
    // 3. Crea commit
    let author = "Default Author";  // per ora semplificato
    let commit_id = create_commit(store_path, &tree_id, parent.as_ref(), author, message)?;
    
    // 4. Aggiorna .ogit/HEAD col nuovo hash
    std::fs::write(&head_path, commit_id.as_str())
        .map_err(|e| format!("Failed to write HEAD: {}", e))?;
    
    println!("{}", commit_id.as_str());
    Ok(())
}

fn cmd_ls_objects(_args: &[String]) -> Result<(), String> {
    let objects_path = Path::new(".ogit/objects");
    
    if !objects_path.exists() {
        return Err("No .ogit repository found".into());
    }
    
    let subdirs = std::fs::read_dir(objects_path)
        .map_err(|e| format!("Failed to read objects: {}", e))?;
    
    for subdir in subdirs {
        let subdir = subdir.map_err(|e| e.to_string())?;
        let subdir_name = subdir.file_name().to_string_lossy().to_string();
        
        if !subdir.path().is_dir() {
            continue;
        }
        
        let files = std::fs::read_dir(subdir.path())
            .map_err(|e| e.to_string())?;
        
        for file in files {
            let file = file.map_err(|e| e.to_string())?;
            let filename = file.file_name().to_string_lossy().to_string();
            let hash = format!("{}{}", subdir_name, filename);
            
            // Leggi tipo oggetto
            let id = OObjectId(hash.clone());
            let obj = read_object(Path::new(".ogit"), &id)?;
            
            println!("{} {}", obj.kind.as_str(), hash);
        }
    }
    
    Ok(())
}

fn cmd_log(_args: &[String]) -> Result<(), String> {
    let store_path = Path::new(".ogit");
    let head_path = store_path.join("HEAD");
    
    if !head_path.exists() {
        return Err("No commits yet".into());
    }
    
    let mut current_hash = std::fs::read_to_string(&head_path)
        .map_err(|e| format!("Failed to read HEAD: {}", e))?
        .trim()
        .to_string();
    
    while !current_hash.is_empty() {
        let id = OObjectId(current_hash.clone());
        let obj = read_object(store_path, &id)?;
        let commit = Commit::deserialize(&obj.data)?;
        
        println!("commit {}", current_hash);
        println!("Author: {}", commit.author);
        println!();
        println!("    {}", commit.message);
        println!();
        
        current_hash = match commit.parent {
            Some(parent) => parent.0,
            None => break,
        };
    }
    
    Ok(())
}