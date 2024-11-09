use clap::Parser;

mod args;

fn main() {
    let args = args::Command::parse();
    println!("Hello, world! {args:?}");
}
