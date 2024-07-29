use crate::cmd::config::InputArgs;
use std::error::Error;
use std::fs;
// use std::path::{Path, PathBuf};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn list_files(args: &InputArgs) -> Result<Vec<PathBuf>, Box<dyn Error>> {
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

fn process_files(
    files: &[PathBuf],
    patterns: &[&str],
) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    // Build the regex patterns in one shot
    let compiled_patterns: Vec<Regex> = patterns
        .iter()
        .map(|&x| Regex::new(x))
        .collect::<Result<_, _>>()?;

    let results = files
        .iter()
        .filter_map(|file| File::open(file).ok().map(BufReader::new)) // attempts to open each file and wraps it in a BufReader. If opening a file fails, it's skipped
        .flat_map(|reader| reader.lines().map_while(Result::ok)) // reads lines from each file, filtering out any errors
        .fold(
            // processes each line, updating the accumulator (a HashMap). For each line, it checks all compiled patterns and updates the HashMap with matches.
            HashMap::new(),
            |mut acc: HashMap<String, Vec<String>>, line| {
                for pattern in &compiled_patterns {
                    if let Some(mat) = pattern.find(&line) {
                        acc.entry(pattern.as_str().to_string())
                            .or_default()
                            .push(mat.as_str().to_string());
                    }
                }
                acc
            },
        );

    Ok(results)
}

pub fn read_files(
    args: &InputArgs,
    patterns: &[&str],
) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let files = list_files(args)?;
    let results = process_files(&files, patterns)?;

    Ok(results)
}
