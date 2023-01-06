use std::io::stdin;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;

use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;

fn main() -> std::io::Result<()> {
    let mut clipboard =
        ClipboardContext::new().or_else(|e| Err(Error::new(ErrorKind::Other, format!("{e}"))))?;
    let mut content = String::new();

    stdin().read_to_string(&mut content)?;

    return clipboard
        .set_contents(content)
        .or_else(|e| Err(Error::new(ErrorKind::Other, format!("{e}"))));
}
