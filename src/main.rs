use std::{fs, io::stdout};
use clap::Parser;
use crossterm::{
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType},
};

#[derive(Parser, Debug)]
#[command(author = "Justin Carruthers", about = "Justin Vim")]
struct Args {
    // File to edit
    filename: String
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let _ = fs::read(&args.filename).expect("Cant read file");
    execute!(
        stdout(),
        Clear(ClearType::All),
        SetForegroundColor(Color::Red),
        Print("Got the file\n"),
    )?;

    Ok(())
}
