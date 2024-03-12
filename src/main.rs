use std::{
    fs,
    io::{stdout, Stdout, Write},
    thread,
    time,
};
use clap::Parser;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

#[derive(Parser, Debug)]
#[command(author = "Justin Carruthers", about = "Justin Vim")]
struct Args {
    // File to edit
    filename: String
}

fn run_editor(output: &mut Stdout) -> std::io::Result<()> {

    thread::sleep(time::Duration::from_secs(2));
    write!(output, "Hey\r\n")?;
    write!(output, "Bye\r\n")?;
    Ok(())
}


fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let _ = fs::read(&args.filename).expect("Cant read file");

    let mut stdout = stdout();

    enable_raw_mode()?;

    match run_editor(&mut stdout) {
        Result::Err(_) => disable_raw_mode()?,
        _ => ()
    }

    disable_raw_mode()?;
    Ok(())
}
