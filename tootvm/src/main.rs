mod token;
// use token::{TokenStream, TokenError};

fn main() {
    // println!("Hello, world!");
    // let ts = token::parse(b"5 + 6").unwrap();
    // println!("{:?}", ts);
    let src = b"5 + 6\n7 + 10\n3 + 4";
    let ts = token::parse(src).unwrap();
    // println!("{:?}", ts.offset_to_line_col(6));
    for i in 0..src.len() {
        println!("{}: {:?}", i, ts.offset_to_line_col(i));
    }
}
