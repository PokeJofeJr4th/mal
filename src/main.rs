#![warn(clippy::pedantic, clippy::nursery)]

use std::io::{stdin, stdout, Write};

use env::{initial_env, Env};
use eval::eval;
use reader::{read_form, read_str};
use types::MalObject;

mod builtins;
mod env;
mod eval;
mod reader;
mod types;

fn main() {
    let env = initial_env();
    loop {
        print!("user>");
        stdout().flush().unwrap();
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        rep(&buf, env.clone());
    }
}

fn rep(r: &str, env: Env) {
    print(&eval(read(r), env));
}

fn read(r: &str) -> MalObject {
    read_form(&mut read_str(r))
}

fn print(r: &MalObject) {
    println!("{r}");
}
