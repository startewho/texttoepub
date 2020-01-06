use regex::Regex; // 1.1.8

fn split_keep<'a>(r: &Regex, text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for c in r.captures_iter(text){
        result.push(c.get(0).map_or("", |m| m.as_str()));
    }
    result
}

fn main() {
    let seperator = Regex::new(r"([a-z]+)").expect("Invalid regex");
    let splits = split_keep(&seperator, "this... is a, test");
    for split in splits {
        println!("\"{}\"", split);
    }
}
