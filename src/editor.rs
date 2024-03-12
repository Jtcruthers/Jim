use std::{
    thread,
    time,
    io::{Stdout, Write},
};

pub fn run_editor(output: &mut Stdout) -> std::io::Result<()> {
    thread::sleep(time::Duration::from_secs(2));
    write!(output, "Hey\r\n")?;
    write!(output, "Bye\r\n")?;
    Ok(())
}
