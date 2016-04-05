mod wichard;
use wichard::Wichard;

use std::io::Read;

fn main() {
    let mut reader = std::io::stdin().wichard();
    let mut buf = String::new();
    
    reader.read_to_string(&mut buf).ok();
    
    println!("{}", buf);
}
