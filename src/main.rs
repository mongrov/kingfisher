extern crate log;
use clap::{Parser, Subcommand};
use config::Config;
use handlebars::Handlebars;
use hocon::{Hocon, HoconLoader};
use log::debug;
use serde_json::{Number, Value};
use std::{collections::BTreeMap, fs};

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
    Hb {
        config: String,
        template: String,
        output: String,
    },
    /// conf (hocon) to yaml file
    Conf {
        config: String,
        output: String,
    },
}

fn hocon_to_json(hocon: Hocon) -> Option<Value> {
    match hocon {
        Hocon::Boolean(b) => Some(Value::Bool(b)),
        Hocon::Integer(i) => Some(Value::Number(Number::from(i))),
        Hocon::Real(f) => Some(Value::Number(
            Number::from_f64(f).unwrap_or(Number::from(0)),
        )),
        Hocon::String(s) => Some(Value::String(s)),
        Hocon::Array(vec) => Some(Value::Array(
            vec.into_iter()
                .map(hocon_to_json)
                .filter_map(|i| i)
                .collect(),
        )),
        Hocon::Hash(map) => Some(Value::Object(
            map.into_iter()
                .map(|(k, v)| (k, hocon_to_json(v)))
                .filter_map(|(k, v)| v.map(|v| (k, v)))
                .collect(),
        )),
        Hocon::Null => Some(Value::Null),
        Hocon::BadValue(_) => None,
    }
}

fn parse_to_json(path: &str) -> String {
    // let hocon = dbg!(HoconLoader::new().no_system().load_file(path).unwrap().hocon()).unwrap();
    let hocon = HoconLoader::new().no_system().load_file(path).unwrap().hocon().unwrap();
    let json: Option<_> = hocon_to_json(hocon);
    serde_yaml::to_string(&json).unwrap()
}

fn main() {
    // RUST_LOG=debug
    env_logger::init();
    let cli = Cli::parse();
    debug!("kf startup");

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Hb {
            config,
            template,
            output,
        } => {
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
            handlebars
                .register_template_file("template", template)
                .unwrap();
            let data = settings
                .try_deserialize::<BTreeMap<String, String>>()
                .unwrap();
            fs::write(output, handlebars.render("template", &data).unwrap())
                .expect("Unable to write to file");
        }
        Commands::Conf {
            config,
            output,
        } => {
            fs::write(output, parse_to_json(&config))
                .expect("Unable to write to file");
        }
    }
}
