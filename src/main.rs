use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    ffi::OsStr,
    fmt,
    fs::File,
    io::{Read, Seek, Write},
};

#[derive(Debug, Serialize, Deserialize)]
struct FileData {
    name: String,
    filenames: Vec<String>,
}

impl fmt::Display for FileData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut file_tree = String::new();
        for file in self.filenames.iter() {
            file_tree.push_str("\n\t");
            file_tree.push_str(file);
        }

        write!(f, "{}:{}", self.name, file_tree)
    }
}

fn list_zip_contents(reader: impl Read + Seek) -> zip::result::ZipResult<Vec<String>> {
    let mut zip = zip::ZipArchive::new(reader)?;
    let mut files: Vec<String> = Vec::new();

    for i in 0..zip.len() {
        let file = zip.by_index(i)?;
        println!("\tFilename: {}", file.name());
        files.push(file.name().to_string());
    }

    Ok(files)
}

fn serialize_to<W: Write, T: ?Sized + Serialize>(
    mut writer: W,
    value: &T,
) -> Result<(), std::io::Error> {
    serde_json::to_writer(&mut writer, value)?;
    writer.write_all(b"\n")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let dir = &args[1];
    let mut zip_files: Vec<FileData> = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension() == Some(OsStr::new("zip")) {
            let file = File::open(&path)?;

            println!("Contents of {:?}:", path);

            let name = path.file_name().unwrap().to_string_lossy().to_string();
            let files = list_zip_contents(file)?;

            let zip_file: FileData = FileData {
                name: name,
                filenames: files,
            };
            zip_files.push(zip_file);
        } else {
            println!("Skipping {:?}", path);
        }
    }

    let json_file = File::create("./zip_file_data.ndjson")?;

    for zip_file in zip_files {
        serialize_to(&json_file, &zip_file)?;

        let j = serde_json::to_string_pretty(&zip_file)?;
        println!("{}", j);
    }

    Ok(())
}
