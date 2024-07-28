// The idea is to build the app to run it as follows:
// parse-files --input_path /somewhere --file_extension log --output_path /elsewhere
// All files in the input path will be parsed, analyzed, and return a csv with results
// $ cargo run -- json --input-path $PWD --file-extesion "json"
// $ cargo run -- text --input-path $PWD --file-extesion "log"

use clap::Parser;
use parse_files::cmd::config;
use parse_files::files::listfiles;

fn main() {
    let cli = config::FileParser::parse();
    let args = cli.input_arguments();

    let files = listfiles::list_files(&args);
    if let Ok(file) = files {
        println!("\nFiles found in {:?}:\n\n{:?}", args.path, file);
    }
}
