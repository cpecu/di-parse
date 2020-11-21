#[macro_use] extern crate pest_derive;

use pest::Parser;

pub mod state;
pub mod parse;
pub mod token;
pub mod grammar;
pub mod models;
pub mod lex;
pub mod ast;

use std::{
    collections::HashMap,
    cell::RefCell,
    fs::File, env, io::prelude::*,
};
use pest::{
    prec_climber::PrecClimber,
    RuleType,
};
pub use parse::*;

pub fn run() {
    let filename = env::args().nth(1)
        .unwrap_or("test.txt".to_string());
    let mut file = File::open(filename).expect("File doesn't exist");
    let mut txtbuf = String::new();
    file.read_to_string(&mut txtbuf).expect("Error reading file");
    println!("{:?}", txtbuf);
    DivParser::get_pairs(txtbuf).unwrap();
}
