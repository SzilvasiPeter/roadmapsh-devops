use chrono::Utc;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::env;
use std::fs::File;
use std::io::Write;
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

    let mut buf = Vec::new();
    {
        let enc = GzEncoder::new(&mut buf, Compression::default());
        let mut tar = Builder::new(enc);
        tar.append_dir_all("", source_dir)?;
        tar.finish()?;
    }

    let now = Utc::now().format("%Y%m%d_%H%M%S");
    let archive = format!("logs_archive_{now}.tar.gz");
    let mut file = File::create(&archive)?;
    file.write_all(&buf)?;

    Ok(())
}
