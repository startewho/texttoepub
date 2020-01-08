use std::fmt;
use std::error::Error;
use std::path::{Path};
use std::fs::File;
use std::io::prelude::*;
use regex::Regex; 

#[derive(Default)]
struct Chapter<'a> {
 name:&'a str,
 contnet:&'a str,
}

impl fmt::Display for Chapter<'_> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} {}", self.name,self.contnet)
    }
}

fn split_keep<'a>(r: &Regex, text: &'a str) -> Vec<Chapter<'a>> {
    let mut result = Vec::new();
    for c in r.captures_iter(text){
        result.push(Chapter{
            name: c.get(0).map_or("", |m| m.as_str()),
            contnet:c.get(0).map_or("", |m| m.as_str()),
    });
    }
    result
}

fn open_text<P:AsRef<Path>>(path: P)->Result<String, Box< dyn Error>>{
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    if let Ok(file) = open_text(".\\1.txt") {
        let seperator = Regex::new(r"([a-z]+)").expect("Invalid regex");
    let splits = split_keep(&seperator, &file);
    for split in splits {
        println!("\"{}\"", split);
    }
    }
    
}
