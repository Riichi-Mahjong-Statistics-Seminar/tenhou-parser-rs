mod enums;
mod round;
mod utils;
mod yaku;
mod serialize;

use glob::glob;
use std::path::Path;
use clap::Parser;
use kdam::tqdm;

#[derive(Parser)]
struct Args {
    #[arg(help = "Input file, directory or glob pattern")]
    input: String,
    #[arg(help = "Output file or directory")]
    output: Option<String>,
}

fn main() {
    stderrlog::new().module(module_path!()).init().unwrap();
    let args = Args::parse();
    if Path::is_file(Path::new(&args.input)) {
        let output = Path::new(&args.output.unwrap_or(args.input.clone())).with_extension("json");
        round::Game::parse_xml_file(&args.input).write_to_json(output);
        log::info!("Done");
        return;
    }
    let (input_glob, output_dir) = if Path::is_dir(Path::new(&args.input)) {
        (format!("{}/*.xml", args.input), args.output.unwrap_or(args.input))
    } else {
        if args.output.is_none() {
            log::error!("Output directory is required when input is a glob pattern");
            return;
        }
        (args.input.clone(), args.output.unwrap_or(args.input))
    };
    let input = glob(&input_glob).unwrap().map(|x| x.unwrap()).collect::<Vec<_>>();
    tqdm!(input.iter()).for_each(|path| {
        let path = path.as_path();
        let output = Path::new(&output_dir).join(path.file_stem().unwrap()).with_extension("json");
        round::Game::parse_xml_file(path).write_to_json(output);
    });
}