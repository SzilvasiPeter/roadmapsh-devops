use chrono::Utc;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::env;
use std::fs::File;
use std::path::Path;
use tar::Builder;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_log_directory>", args[0]);
        std::process::exit(1);
    }

    let source_dir = &args[1];
    if !Path::new(source_dir).exists() {
        eprintln!("Error: The directory '{source_dir}' does not exist.");
        std::process::exit(1);
    }

    let now = Utc::now().format("%Y%m%d_%H%M%S");
    let archive = format!("logs_archive_{now}.tar.gz");
    let file = File::create(&archive)?;
    let enc = GzEncoder::new(file, Compression::default());
    let mut tar = Builder::new(enc);
    tar.append_dir_all("", source_dir)?;
    tar.finish()?;

    Ok(())
}
