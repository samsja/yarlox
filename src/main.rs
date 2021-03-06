mod scanner;
mod token;
mod error;

use scanner::scan_source;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::exit;

fn main() {
    println!("Welcome to the yarlox interpreter");

    let args: Vec<String> = env::args().collect();
    let num_args = args.len();
    if num_args > 2 {
        panic!("Usage: yarlox [script]");
    } else if num_args == 2 {
        run_file(&args[1]).expect("Failed to run");
    } else {
        run_prompt();
    }
}

fn run_file(script_fn: &String) -> Result<(), io::Error> {
    println!("script : {}", script_fn);
    let mut file = File::open(script_fn)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    run(&contents);
    Ok(())
}

fn run_prompt() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    if input.len() == 1 {
        exit(0);
    } else {
        run(&input);
    }
}

fn run(source: &String) {
    println!("run {source}");
    println!("{:?}", scan_source(source));
}

