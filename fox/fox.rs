use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    kilo: i32
}

fn main() {
    let args = Args::parse();
    dbg!(&args);
}
