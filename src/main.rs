extern crate log;
use clap::{Parser, Subcommand};
use log::info;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// handlebar templates 
    Hb { filename: String },
    /// Test
    Test,
}

fn main() {
    // RUST_LOG=debug
    env_logger::init();
    let cli = Cli::parse();
    info!("kf startup");

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Hb { filename } => {
            println!("File system called with {filename}");
        }
        _ => {
            // timely::example(|scope| {
            //     (0..10)
            //         .to_stream(scope)
            //         .inspect(|x| println!("seen: {:?}", x));
            // });
        }
    }}
