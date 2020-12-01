extern crate my_rust_eval;

use my_rust_eval::my_eval;

fn main() {
    let mut args = std::env::args();
    args.next();
    match args.next() {
        None => panic!("No argument"),
        Some(str) => println!("{}", my_eval::my_eval(str.as_str()))
    }
}
