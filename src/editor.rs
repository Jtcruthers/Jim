use std::io::{Stdout, Write};
use crossterm::event::{read, Event, KeyCode};

const EXIT_EVENT: KeyCode = KeyCode::Char('q');

pub fn run_editor(output: &mut Stdout) -> std::io::Result<()> {
    write!(output, "Hey\r\n")?;
    loop {
        let input = read()?;
        match input {
            Event::Key(key) => match key.code {
                EXIT_EVENT => break,
                _ => ()
            },
            _ => ()
        }
    }
    write!(output, "Bye\r\n")?;

    Ok(())
}
