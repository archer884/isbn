#![feature(iter_arith)]

mod isbn;
use isbn::Isbn;

pub fn main() {
    for candidate in std::env::args().skip(1) {
        match candidate.parse::<Isbn>() {
            Ok(isbn) => println!("Valid: {}", isbn),
            Err(e) => println!("Invalid: {:?}", e),
        }
    }
}
