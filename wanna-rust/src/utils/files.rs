use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},

    process::Command
};

use lib::config::control::CAN_HIDE_ITEMS;

pub fn create_folder(name: &str, path: &Path) -> io::Result<PathBuf> {
    let folder: PathBuf = path.join(name);
    if !folder.exists() {
        fs::create_dir_all(&folder)?;
    }

    Ok(folder)
}

pub fn create_item(name: &str, path: &Path) -> io::Result<PathBuf> {
    let item: PathBuf = path.join(name);
    if !item.exists() {
        File::create(&item)?;
    }

    Ok(item)
}

pub fn write_file(path: &Path, content: &str) -> io::Result<()> {
    let mut item: File = File::create(path)?;
    item.write_all(content.as_bytes())?;
    item.flush()?;

    Ok(())
}

pub fn delete_item(path: &Path) -> io::Result<()> {
    if !path.exists() {
        return Ok(());
    }

    if path.is_dir() {
        fs::remove_dir_all(path)?;
    } else {
        fs::remove_file(path)?;
    }

    Ok(())
}

pub fn hide_item(path: &Path, system: bool) -> io::Result<()> {
    if !CAN_HIDE_ITEMS {
        return Ok(());
    }

    let mut args: Vec<String> = vec!["+h".to_string()];
    if system {
        args.push("+s".to_string());
    }
    args.push(path.to_str().unwrap().to_string());

    Command::new("attrib")
        .args(&args)
        .output()?;

    Ok(())
}

pub fn walk_dir<F>(path: &Path, callback: &F) where F: Fn(&Path), {
    let Ok(files) = fs::read_dir(path) else {
        eprintln!("failed to read dir: {}", path.display());
        return;
    };

    for file in files {
        let Ok(entry) = file else {
            eprintln!("failed to read {}: {}", path.display(), file.unwrap_err());
            continue;
        };

        let path: PathBuf = entry.path();
        if path.is_dir() {
            walk_dir(&path, callback);
        } else {
            callback(&path);
        }
    }
}