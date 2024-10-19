use clap::Parser;
use std::error::Error;
use std::path::Path;
use std::{fs, io};
use annotation_format_converter::Converter;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "source file, like .xml or .txt, or a folder")]
    source: String,
    #[arg(short, long, help = "dest folder")]
    dest: String,
    #[arg(
        short,
        long,
        help = "classes file, like classes.txt, one class per line"
    )]
    classes: String,
}

fn parse_classes(classes_file: &str) -> Result<Vec<String>, io::Error> {
    let classes = fs::read_to_string(classes_file)?
        .lines()
        .map(|v| v.to_string())
        .collect();
    Ok(classes)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let converter = Converter::new(parse_classes(args.classes.as_str())?);
    let source = Path::new(args.source.as_str());
    let dest = Path::new(args.dest.as_str());
    if !source.exists() {
        eprintln!("Error: file or folder not exists: {:?}", source);
        return Ok(());
    }
    if source.is_file() {
        if source.extension().unwrap() == "xml" {
            let filename = source.file_stem().unwrap().to_str().unwrap();
            let _ = converter
                .voc_to_yolo(
                    source.to_str().unwrap(),
                    dest.join(Path::new(format!("{filename}.txt").as_str()))
                        .to_str()
                        .unwrap(),
                )
                .map_err(|e| eprintln!("Error: {}", e.to_string()));
        }
    }
    if source.is_dir() {
        for dir in source.read_dir() {
            for dir in dir {
                let path = dir?.path();
                if path.is_file() {
                    let filename = path.file_stem().unwrap().to_str().unwrap();
                    match path.extension() {
                        Some(extension) => {
                            if extension.to_str().unwrap() == "xml" {
                                let _ = converter
                                    .voc_to_yolo(
                                        path.to_str().unwrap(),
                                        dest.join(format!("{filename}.txt")).to_str().unwrap(),
                                    )
                                    .map_err(|e| eprintln!("Error: {}", e.to_string()));
                            }
                        }
                        None => {}
                    }
                }
            }
        }
    }

    Ok(())
}
