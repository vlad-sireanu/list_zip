use std::{
    fs::{read_dir, File},
    io::{Read, Seek},
};

fn list_zip_contents(reader: impl Read + Seek) {
    if let Ok(mut zip) = zip::ZipArchive::new(reader) {
        for i in 0..zip.len() {
            if let Ok(file) = zip.by_index(i) {
                println!("Filename: {}", file.name());
            }
        }
    }
}

fn search_in(directory: &str) {
    println!("\n\nSearching in {}", directory);

    if let Ok(entries) = read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(file_type) = path.extension() {
                    if file_type == "zip" {
                        println!("\nArchive Name: {}", path.display());
                        if let Ok(file) = File::open(&path) {
                            list_zip_contents(file);
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    for i in 1..args.len() {
        search_in(&args[i]);
    }
}
