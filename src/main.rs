#![warn(clippy::pedantic, clippy::nursery)]

use std::io::stdin;

fn main() {
    loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        println!("{}", rep(buf));
    }
}

fn rep(r: String) -> String {
    print(eval(read(r)))
}

fn read(r: String) -> String {
    r
}

fn eval(r: String) -> String {
    r
}

fn print(r: String) -> String {
    r
}
