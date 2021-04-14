#![feature(const_mut_refs)]
#![feature(const_fn_fn_ptr_basics)]
mod language;

#[cfg(feature = "log_level_debug")]
use crate::language::debug::Debug;
use crate::language::vm::VM;

use crate::language::compiler::Compiler;

use std::{
    env, fs,
    io::{self, Write},
    process,
};

// https://github.com/felipesabino/lox-rust/blob/master/src/main.rs
fn main() {
    welcome_message();
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => repl(),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: cargo run [path]");
            process::exit(64);
        }
    }
}

fn repl() {
    let mut vm = VM::new();
    let mut input = String::new();
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout!");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        vm.interpret(&input).ok();
    }
}

fn run_file(path: &str) {
    let mut vm = VM::new();
    let file = fs::read_to_string(path);
    match file {
        Ok(input) => vm.interpret(&input).ok(),
        Err(_) => {
            println!("Failed to read file.");
            process::exit(74);
        }
    };
}

fn welcome_message() {
    println!("============================================");
    println!("||              ::Welcome::               ||");
    println!("||       TypeScript Native PoC v0.1       ||");
    println!("============================================");
    println!("");
}
