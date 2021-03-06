use std::fmt;

#[derive(Default)]
pub struct Chapter<'a> {
    name: &'a str,
    content: &'a str,
    start: usize,
    end: usize,
    index: usize,
}

pub struct Meta {
    pub key: String,
    pub value: String,
}

#[derive(Default)]
pub struct Book<'a> {
    chapters: Vec<Chapter<'a>>,
    metas: Vec<Meta>,
    source: &'a str,
    target:&'a str,
}

impl Chapter<'_> {
    pub fn new<'a>(n: &'a str, c: &'a str, s: usize, e: usize, i: usize) -> Chapter<'a> {
        Chapter {
            name: n,
            content: c,
            start: s,
            end: e,
            index: i,
        }
    }
    pub fn get_name<'a>(&'a self) -> &'a str {
        self.name
    }
    pub fn get_content<'a>(&'a self) -> &'a str {
        self.content
    }
    pub fn get_start<'a>(&'a self) -> usize {
        self.start
    }
    pub fn get_end<'a>(&'a self) -> usize {
        self.end
    }
    pub fn get_index<'a>(&'a self) -> usize {
        self.index
    }
}

impl Book<'_> {
    pub fn new<'a>(m: Vec<Meta>, c: Vec<Chapter<'a>>, s: &'a str,t:&'a str) -> Book<'a> {
        Book {
            metas: m,
            chapters: c,
            source: s,
            target:t
        }
    }

    pub fn get_metas(&self) -> &[Meta] {
        &self.metas
    }
    pub fn get_chapter<'a>(&'a self) -> &[Chapter<'a>] {
        &self.chapters
    }

    pub fn get_source<'a>(&'a self) -> &'a str {
        self.source
    }

    pub fn get_target<'a>(&'a self) -> &'a str {
        self.target
    }
}

impl fmt::Display for Chapter<'_> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} {}", self.name, self.content)
    }
}
