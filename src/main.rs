use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod epub;
mod meta;
fn split_keep<'a>(r: &Regex, text: &'a str) -> Vec<meta::Chapter<'a>> {
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

fn main() {
    if let Ok(file) = open_text(".\\1.txt") {
        let seperator = Regex::new(r"(?m)(^\s*[第卷][0123456789一二三四五六七八九十零〇百千两]*[章回部节集卷].*)").expect("Invalid regex");
        let splits = split_keep(&seperator, &file);
        let book = meta::Book::new(Vec::new(), splits,&file);
        epub::gen_epub(book);
    }
}
