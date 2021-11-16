use std::io::{stdout};
use crossterm::Result;
mod console;

fn main() -> Result<()> {
    let _ = console::clear(stdout());
    let _ = console::writeln("ABC");

    Ok(())
}