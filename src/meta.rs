use std::fmt;

#[derive(Default)]
pub struct Chapter<'a> {
 name:&'a str,
 contnet:&'a str,
}


impl Chapter<'_> {
pub fn  new<'a>(n:&'a str,c:&'a str) ->Chapter<'a> {
    Chapter{
        name:n,
        contnet:c,
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
        write!(f, "{} {}", self.name,self.contnet)
    }
}
