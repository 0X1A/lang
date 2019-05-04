#[macro_use]
extern crate clap;

use clap::App;
use lang::{error::*, lang::*};

fn main() -> Result<(), LangError> {
    let clap_config = load_yaml!("../lang.yaml");
    let arg_matches = App::from_yaml(clap_config).get_matches();
    if let Some(file_path) = arg_matches.value_of("run_file") {
        let mut logging_level: u64 = 0;
        if arg_matches.is_present("debug") {
            logging_level += 1;
            Lang::setup_logging(logging_level)?;
            Lang::new(Some(&Lang::read_file(file_path.to_string())?)).run()?;
        } else if arg_matches.is_present("print_ast") {
            Lang::new(Some(&Lang::read_file(file_path.to_string())?)).print_ast()?;
        } else {
            Lang::new(Some(&Lang::read_file(file_path.to_string())?)).run()?;
        }
    }
    Ok(())
}
