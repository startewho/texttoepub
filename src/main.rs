#![allow(dead_code)]
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod epub;
mod meta;
fn split_chapters<'a>(r: &Regex, text: &'a str) -> Vec<meta::Chapter<'a>> {
    let mut result = Vec::new();
    
    for c in r.captures_iter(text) {
        let title=c.get(0).unwrap();
        let chapter=meta::Chapter::new(title.as_str(),title.as_str(),title.start(),title.end());
        result.push(chapter);
    }
    result
}

fn open_text<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn clear_text(source:&str)->String
{
   let re= Regex::new(r"<[^>]+>").expect("Invalid regex");
   let text= re.replace_all(source,"").into_owned();
   text    
}

fn main() {
    if let Ok(source) = open_text(".\\1.txt") {
        let text=clear_text(&source);
        let seperator = Regex::new(r"(?m)(^\s*[第卷][0123456789一二三四五六七八九十零〇百千两]*[章回部节集卷].*)").expect("Invalid regex");
        let splits = split_chapters(&seperator, &text);
        let book = meta::Book::new(Vec::new(), splits,&text);
       let result=epub::gen_epub(book);
    }
}
