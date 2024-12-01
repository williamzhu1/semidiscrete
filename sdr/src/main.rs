use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use clap::Parser as ClapParser;
use jagua_rs::entities::solution::{self, Solution};
use log::{error, warn};
use mimalloc::MiMalloc;
use rand::prelude::SmallRng;
use rand::SeedableRng;

use jagua_rs::io::parser;
use jagua_rs::io::parser::Parser;
use jagua_rs::util::polygon_simplification::PolySimplConfig;
use sdr::io::cli::Cli;
use sdr::io::json_output::JsonOutput;
use sdr::io::layout_to_svg::s_layout_to_svg;
use sdr::sdr_config::SDRConfig;
use sdr::{io, EPOCH};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

//more efficient allocator
fn main() {
    let args = Cli::parse();
    io::init_logger(args.log_level);

    let config = match args.config_file {
        None => {
            warn!("No config file provided, use --config-file to provide a custom config");
            warn!(
                "Falling back default config:\n{}",
                serde_json::to_string(&SDRConfig::default()).unwrap()
            );
            SDRConfig::default()
        }
        Some(config_file) => {
            let file = File::open(config_file).unwrap_or_else(|err| {
                panic!("Config file could not be opened: {}", err);
            });
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_else(|err| {
                error!("Config file could not be parsed: {}", err);
                error!("Omit the --config-file argument to use the default config");
                panic!();
            })
        }
    };

    let json_instance = io::read_json_instance(args.input_file.as_path());
    let poly_simpl_config = match config.poly_simpl_tolerance {
        Some(tolerance) => PolySimplConfig::Enabled { tolerance },
        None => PolySimplConfig::Disabled,
    };

    let parser = Parser::new(poly_simpl_config, config.cde_config, true);
    let instance = parser.parse(&json_instance);

    let rng = match config.prng_seed {
        Some(seed) => SmallRng::seed_from_u64(seed),
        None => SmallRng::from_entropy(),
    };

    if !args.solution_folder.exists() {
        fs::create_dir_all(&args.solution_folder).unwrap_or_else(|_| {
            panic!(
                "could not create solution folder: {:?}",
                args.solution_folder
            )
        });
    }

    let input_file_stem = args.input_file.file_stem().unwrap().to_str().unwrap();

}
