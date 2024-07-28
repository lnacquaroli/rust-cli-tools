use crate::cmd::config::InputArgs;
use std::error::Error;
use std::fs;
// use std::path::{Path, PathBuf};
use std::path::PathBuf;

pub fn list_files(args: &InputArgs) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let dir = args.path.clone();
    let extension = args.extension.as_str();

    let mut files = Vec::new();
    let entries = fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == extension) {
            files.push(path);
        }
    }

    Ok(files)
}

// pub fn list_files<P: AsRef<Path>>(dir: P, extension: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
//     let mut files = Vec::new();
//     let entries = fs::read_dir(dir)?;

//     for entry in entries {
//         let entry = entry?;
//         let path = entry.path();

//         if path.extension().map_or(false, |ext| ext == extension) {
//             files.push(path);
//         }
//     }

//     Ok(files)
// }
// fn main() -> Result<(), Box<dyn Error>> {
//     let path = ".";
//     let extension = "rs";
//     let files = list_files(path, extension)?;

//     for file in files {
//         println!("{:?}", file);
//     }

//     Ok(())
// }

// use glob::glob;
// use std::error::Error;
// use std::path::{Path, PathBuf};

// fn list_files_with_extension_glob<P: AsRef<Path>>(
//     path: P,
//     extension: &str,
// ) -> Result<Vec<PathBuf>, Box<dyn Error>> {
//     let mut files = Vec::new();
//     let search_pattern = format!("{}/**/*.{}", path.as_ref().display(), extension);

//     for entry in glob(&search_pattern)? {
//         match entry {
//             Ok(path) => files.push(path),
//             Err(e) => eprintln!("Error reading entry: {:?}", e),
//         }
//     }

//     Ok(files)
// }

// fn main() -> Result<(), Box<dyn Error>> {
//     let path = ".";
//     let extension = "rs";
//     let files = list_files_with_extension_glob(path, extension)?;

//     for file in files {
//         println!("{:?}", file);
//     }

//     Ok(())
// }
