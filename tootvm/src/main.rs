mod token;
use token::{TokenStream, TokenError};

fn main() {
    println!("Hello, world!");
    let ts = token::parse(b"5 + 6").unwrap();
    println!("{:?}", ts);
}
