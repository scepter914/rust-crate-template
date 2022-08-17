use std::fs::File;
use std::io::Read;

use carrot_utils::logger;
use clap::Parser;
use log::LevelFilter;
use serde_derive::Deserialize;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short = 'c', long = "config_path")]
    config_path: String,
    #[clap(short = 'l', long = "log_level", default_value = "Info")]
    log_level: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    param_1: i32,
    param_2: i32,
}

fn main() {
    // arg
    let args = Args::parse();

    // set config
    let mut toml_str = String::new();
    let config_path = args.config_path;
    File::open(&config_path)
        .and_then(|mut f| f.read_to_string(&mut toml_str))
        .unwrap();
    let config: Config = toml::from_str(&toml_str).unwrap();
    println!("{:#?}", config);
    println!("{}", config.param_1);

    // set logger
    let log_level = logger::get_log_level(&args.log_level);

    let logger = logger::Logger::new(
        "./data/{TIME_SEC}",
        "log_{TIME_SEC}.txt",
        LevelFilter::Debug,
        LevelFilter::Debug,
    );
}
