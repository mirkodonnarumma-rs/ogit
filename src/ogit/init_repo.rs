use std::{
    fs::{self, File},
    io::{Error, Write},
    path::{Path},
};

pub fn init_repo() -> Result<(), Error> {
    let root = Path::new(".ogit");

    if root.exists() {
        eprintln!("Cartella già presente.");
        return Ok(());
    }

    fs::create_dir_all(root)?;

    // objects
    fs::create_dir_all(root.join("objects"))?;

    // refs/heads
    let heads_path = root.join("refs/heads");
    fs::create_dir_all(&heads_path)?;

    // HEAD file
    let head_file = heads_path.join("HEAD");
    create_head(&head_file)?;

    Ok(())
}

fn create_head(path: &Path) -> Result<(), Error> {
    let content = b"ref: refs/heads/master\n";

    if path.exists() {
        eprintln!("File già presente.");
        return Ok(());
    }

    let mut f = File::create(path)?;
    f.write_all(content)?;
    Ok(())
}
