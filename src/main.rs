
use std::error::Error;
use std::path::{Path};
use std::fs::File;
use std::io::prelude::*;
use regex::Regex; 


mod meta;
mod epub;
fn split_keep<'a>(r: &Regex, text: &'a str) -> Vec<meta::Chapter<'a>> {
    let mut result = Vec::new();
    for c in r.captures_iter(text){
        let name=c.get(0).map_or("", |m| m.as_str());
        let content=c.get(0).map_or("", |m| m.as_str());
        let chapter=meta::Chapter::new(name, content);
        result.push(chapter);
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
    let chapter =meta::Chapter::new("f", "f");
       epub::gen_epub(chapter);
    }
    
}
