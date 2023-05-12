extern crate log;
use clap::{Parser, Subcommand};
use config::Config;
use handlebars::Handlebars;
use log::debug;
use std::collections::BTreeMap;
// use std::collections::HashMap;

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
    Hb { config: String, template: String },
    /// Test
    Test,
}

fn main() {
    // RUST_LOG=debug
    env_logger::init();
    let cli = Cli::parse();
    debug!("kf startup");

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Hb { config , template} => {
            // create the handlebars registry
            let mut handlebars = Handlebars::new();
            let settings = Config::builder()
                .add_source(config::File::with_name(config))
                .build()
                .unwrap();
            // println!(
            //     "{:?}",
            //     settings
            //         .try_deserialize::<BTreeMap<String, String>>()
            //         .unwrap()
            // );
            handlebars.register_template_file("template", template).unwrap();
            let data = settings.try_deserialize::<BTreeMap<String, String>>().unwrap();
            println!("{}",  handlebars.render("template", &data).unwrap());
        }
        _ => {
            println!("Nothing here");
            // The data type should implements `serde::Serialize`
            // let mut data = BTreeMap::new();
            // data.insert("world".to_string(), "世界!".to_string());
            // println!("render {}", handlebars.render("t1", &data).unwrap());
        }
    }
}
