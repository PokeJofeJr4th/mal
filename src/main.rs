#![warn(clippy::pedantic, clippy::nursery)]

use std::io::{stdin, stdout, Write};

use eval::{eval, initial_env, Env};
use reader::{read_form, read_str};
use types::MalObject;

mod eval;
mod reader;
mod types;

fn main() {
    let mut env = initial_env();
    loop {
        print!("user>");
        stdout().flush().unwrap();
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        rep(&buf, &mut env);
    }
}

fn rep(r: &str, env: &mut Env) {
    print(&eval(read(r), env));
}

fn read(r: &str) -> MalObject {
    read_form(&mut read_str(r))
}

fn print(r: &MalObject) {
    println!("{r}");
}
