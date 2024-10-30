#![warn(clippy::pedantic, clippy::nursery)]

use std::io::{stdin, stdout, Write};

use reader::{read_form, read_str};
use types::MalObject;

mod reader;
mod types;

fn main() {
    loop {
        print!("user>");
        stdout().flush().unwrap();
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        rep(&buf);
    }
}

fn rep(r: &str) {
    print(&eval(read(r)));
}

fn read(r: &str) -> MalObject {
    read_form(&mut read_str(r))
}

fn eval(r: MalObject) -> MalObject {
    r
}

fn print(r: &MalObject) {
    println!("{r}");
}
