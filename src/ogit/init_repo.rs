use std::{fs, io::Error, path::PathBuf};

pub fn init_repo() -> Result<Vec<PathBuf>, Error> {
    let mut path_to_be_created = Vec::new();

    // Create object directory
    let mut objects = PathBuf::from(".ogit/");
    objects.push("objects");
    fs::create_dir_all(&objects)?;
    path_to_be_created.push(objects);
    
    // Create refs directory
    let mut refs = PathBuf::from(".ogit/");
    refs.push("refs");
    fs::create_dir_all(&refs)?;
    path_to_be_created.push(refs);

    // Create heads inside refs directory
    let mut refs = PathBuf::from(".ogit/");
    refs.push("refs/heads");
    fs::create_dir_all(&refs)?;
    path_to_be_created.push(refs);

    // Create refs directory
    let mut head = PathBuf::from(".ogit/");
    head.push("refs");
    fs::create_dir_all(&head)?;
    path_to_be_created.push(head);

    Ok(path_to_be_created)
}