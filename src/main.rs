#[macro_use]
extern crate clap;

use clap::App;
use lang::{error::*, lang::*};
use std::result::Result;

fn main() -> Result<(), LangError> {
    let clap_config = load_yaml!("../lang.yaml");
    let arg_matches = App::from_yaml(clap_config).get_matches();
    if let Some(file_path) = arg_matches.value_of("run_file") {
        Lang::setup_logging(0)?;
        match Lang::read_file(file_path.to_string()) {
            Err(e) => println!("{}", e),
            Ok(content) => {
                let result;
                if arg_matches.is_present("print_ast") {
                    result = Lang::new(Some(&content)).print_ast();
                } else if arg_matches.is_present("print_tokens") {
                    result = Lang::new(Some(&content)).print_tokens();
                } else {
                    result = Lang::new(Some(&content)).run();
                }
                match result {
                    Ok(_) => (),
                    Err(e) => println!("{}", e),
                }
            }
        };
    }
    Ok(())
}
