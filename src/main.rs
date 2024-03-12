use std::{
    fs,
    thread,
    time,
    io::stdout
};
use clap::Parser;
use crossterm::{
    execute,
    terminal::{Clear, ClearType, SetSize, ScrollUp, size},
};

#[derive(Parser, Debug)]
#[command(author = "Justin Carruthers", about = "Justin Vim")]
struct Args {
    // File to edit
    filename: String
}

fn show_editor() -> std::io::Result<()> {
    execute!(
        stdout(),
        Clear(ClearType::All),
        SetSize(10, 10),
        ScrollUp(5)
    )?;

    thread::sleep(time::Duration::from_secs(5));

    Ok(())
}


fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let (cols, rows) = size()?;

    let _ = fs::read(&args.filename).expect("Cant read file");

    show_editor()?;

    execute!(
        stdout(),
        SetSize(cols, rows)
    )?;

    Ok(())
}
