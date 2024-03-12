use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Justin Carruthers", about = "Justin Vim")]
struct Args {
    // File to edit
    filename: String
}

fn main() {
    let args = Args::parse();

    ()
}
