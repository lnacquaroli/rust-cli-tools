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
    let patterns = vec![r"name", r"codegen"];

    let results = listfiles::read_files(&args, &patterns);

    for (pattern, matches) in results.unwrap() {
        println!("Pattern: {}", pattern);
        for mat in matches {
            println!("  Match: {}", mat);
        }
    }
}
