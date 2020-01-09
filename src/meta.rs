use std::fmt;

#[derive(Default)]
pub struct Chapter<'a> {
 pub name:&'a str,
 pub content:&'a str,
}

pub struct Meta{
   pub key:String,
   pub value:String,
}

#[derive(Default)]
pub struct Book<'a>{
   pub chapters:Vec<Chapter<'a>>,
   pub metas:Vec<Meta>,

}

impl Chapter<'_> {
pub fn  new<'a>(n:&'a str,c:&'a str) ->Chapter<'a> {
    Chapter{
        name:n,
        content:c,
    }
}
}

impl Book<'_> {
    pub fn  new<'a>(m:Vec<Meta>,c:Vec<Chapter<'a>>) ->Book<'a> {
        Book{
            metas:m,
            chapters:c,
        }
    }
    }


impl fmt::Display for Chapter<'_> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} {}", self.name,self.content)
    }
}
