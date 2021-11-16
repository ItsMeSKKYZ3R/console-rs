use std::{fmt::Display, io::Stdout};
use std::io::Write;

use crossterm::{Result, cursor::{MoveTo}, execute, terminal::{Clear, ClearType}};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[allow(dead_code, unused)]
pub fn clear(mut stdout: Stdout) -> Result<()> {
    let _ = execute!(
        stdout,
        Clear(ClearType::All),
        MoveTo(0, 0)
    );

    Ok(())
}

#[allow(dead_code, unused)]
pub fn write<T: Display>(data: T) -> Result<()> {
    print!("{}", data);

    Ok(())
}

#[allow(dead_code, unused)]
pub fn writeln<T: Display>(data: T) -> Result<()> {
    println!("{}", data);

    Ok(())
}

#[allow(dead_code, unused)]
pub fn write_color<T: Display>(text: T, color: Color, inline: bool) -> Result<()> {
    if inline {
        let mut stdout: StandardStream = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(color)));
        writeln!(&mut stdout, "{}", text);
    } else {
        let mut stdout: StandardStream = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(color)));
        write!(&mut stdout, "{}", text);
    }

    Ok(())
}