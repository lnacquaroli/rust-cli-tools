use clap::{command, Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Default, Clone)] // PartialEq, Hash, Eq, PartialOrd, Ord
pub struct InputArgs {
    pub extension: String,
    pub path: PathBuf,
}

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about("\n\nApplication to parse files inside a folder")
)]
pub struct FileParser {
    #[command(subcommand)]
    pub file_type: FileType,
}

#[derive(Debug, Subcommand)]
pub enum FileType {
    /// Create a log/text file parser
    Text(TextSubcommand),
    /// Create a json file parser
    Json(JsonSubcommand),
}

#[derive(Debug, Args)]
pub struct TextSubcommand {
    /// Input path with the files to be analyzed
    #[arg(short = 'p', aliases = ["path", "inputpath", "input-path"], long)]
    pub input_path: PathBuf,
    /// Extension of the files to be analyzed
    #[arg(short = 'e', aliases = ["file-extension", "extension", "ext", "format"], long, default_value = "log")]
    pub file_extension: String,
}

#[derive(Debug, Args)]
pub struct JsonSubcommand {
    /// Input path with the files to be analyzed
    #[arg(short = 'p', aliases = ["path", "inputpath", "input-path"], long)]
    pub input_path: PathBuf,
    /// Extension of the files to be analyzed
    #[arg(short = 'e', aliases = ["file-extension", "extension", "ext", "format"], long, default_value = "json")]
    pub file_extension: String,
}

impl FileParser {
    pub fn input_arguments(&self) -> InputArgs {
        match &self.file_type {
            FileType::Text(x) => {
                println!(
                    "Parse text files with parameters: \n - Extension: {:#?} \n - Directory: {:#?}",
                    x.file_extension, x.input_path
                );
                InputArgs {
                    path: x.input_path.clone(),
                    extension: x.file_extension.clone(),
                }
            }
            FileType::Json(x) => {
                println!(
                    "Parse json files with parameters: \n - Extension: {:#?} \n - Directory: {:#?}",
                    x.file_extension, x.input_path
                );
                InputArgs {
                    path: x.input_path.clone(),
                    extension: x.file_extension.clone(),
                }
            }
        }
    }
}
