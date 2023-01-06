use std::fs::File;
use std::io::stdin;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::path::PathBuf;

use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// Copy a file's content to the system's clipboard
struct Cli {
    /// Path of the file contents to copy.
    path: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    let mut content = String::new();

    match args.path {
        Some(path) => {
            File::open(path)?.read_to_string(&mut content)?;
        }
        None => {
            stdin().read_to_string(&mut content)?;
        }
    }

    let mut clipboard =
        ClipboardContext::new().or_else(|e| Err(Error::new(ErrorKind::Other, format!("{e}"))))?;
    return clipboard
        .set_contents(content)
        .or_else(|e| Err(Error::new(ErrorKind::Other, format!("{e}"))));
}
