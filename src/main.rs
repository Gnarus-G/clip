use std::fmt::Display;
use std::fs::File;
use std::io::stdin;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::path::PathBuf;

use clap::Subcommand;
use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// Copy a file's content to the system's clipboard
struct Cli {
    /// Path of the file contents to copy.
    path: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Read from the clipboard and print to stdout.
    READ,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    let mut clipboard = ClipboardContext::new().or_else(handle_clipboard_error)?;

    let mut content = String::new();

    return match args.command {
        Some(Command::READ) => {
            println!(
                "{}",
                clipboard.get_contents().or_else(handle_clipboard_error)?
            );
            Ok(())
        }
        None => {
            match args.path {
                Some(path) => {
                    File::open(path)?.read_to_string(&mut content)?;
                }
                None => {
                    stdin().read_to_string(&mut content)?;
                }
            }

            clipboard
                .set_contents(content)
                .or_else(handle_clipboard_error)
        }
    };
}

fn handle_clipboard_error<E: Display, R>(e: E) -> std::io::Result<R> {
    return Err(Error::new(ErrorKind::Other, format!("{e}")));
}
