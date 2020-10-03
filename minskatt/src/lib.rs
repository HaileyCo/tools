#[macro_use]
extern crate lalrpop_util;

pub mod sru;
pub mod sru_parser;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn validate_string(file: &str) -> u8{
    let sru = sru_parser::parse(file.to_string());
    match sru {
	Ok(_) => 0,
	Err(_) => 1
    }
}
