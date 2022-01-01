extern crate my_rust_eval;

fn println_my_eval_dynamic(s: &String) {
    println!("{}", my_rust_eval::my_eval_dynamic::my_eval(s.as_str()))
}

fn println_my_eval_static(s: &String) {
    println!("{}", my_rust_eval::my_eval_static::my_eval(s.as_str()))
}

fn main() {
    let mut args = std::env::args();
    args.next();
    match args.next() {
        None => panic!("No argument"),
        Some(str) => match args.next() {
            None => println_my_eval_static(&str),
            Some(t) if t == "static" => println_my_eval_static(&str),
            Some(t) if t == "dynamic" => println_my_eval_dynamic(&str),
            _ => panic!("Optional second parameter is not static nor dynamic")
        }
    }
}
