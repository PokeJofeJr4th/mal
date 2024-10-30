use std::sync::LazyLock;

use regex::Regex;

use crate::types::MalObject;

pub struct Reader<'a> {
    toks: Vec<&'a str>,
    position: usize,
}

impl<'a> Iterator for Reader<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let tok = self.toks.get(self.position).copied();
        self.position += 1;
        tok
    }
}

impl<'a> Reader<'a> {
    pub fn peek(&self) -> Option<&'a str> {
        self.toks.get(self.position).copied()
    }
}

/// This function will peek at the first token in the Reader object and switch on the first character of that token.
/// If the character is a left paren then read_list is called with the Reader object.
/// Otherwise, read_atom is called with the Reader Object. The return value from read_form is a mal data type.
pub fn read_form(reader: &mut Reader) -> MalObject {
    match reader.peek() {
        Some("(") => read_list(reader),
        _ => read_atom(reader),
    }
}

pub fn read_list(reader: &mut Reader) -> MalObject {
    reader.next();
    let mut items = Vec::new();
    while !matches!(reader.peek(), Some(")") | None) {
        items.push(read_form(reader));
    }
    reader.next();
    MalObject::List(items)
}

pub fn read_atom(reader: &mut Reader) -> MalObject {
    let Some(s) = reader.next() else { todo!() };
    s.parse::<i32>()
        .map_or_else(|_| MalObject::Symbol(s.to_string()), MalObject::Int)
}

pub fn read_str(str: &str) -> Reader {
    Reader {
        toks: tokenize(str),
        position: 0,
    }
}

pub fn tokenize(str: &str) -> Vec<&str> {
    static TOKENIZE_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#)
            .unwrap()
    });
    TOKENIZE_RE
        .captures_iter(str)
        .map(|capture| capture.get(1).unwrap().as_str())
        .collect()
}
