mod wichard;
use wichard::Wichard;

use std::io::{Cursor, Read};

fn main() {
    let mut reader = Cursor::new("Hello, world! How are you?").wichard();
    let mut buf = String::new();
    
    reader.read_to_string(&mut buf).ok();
    
    println!("{}", buf);
}
