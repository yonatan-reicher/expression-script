mod parse;
mod ast;
mod reduce;

fn main() {
    let code = "galord123 -> guyyyy";
    match parse::parse(code) {
        Ok(x) => {
            println!("Parsing successful!");
            println!("{:?}", x);
        },
        Err(()) => {
            println!("Failed parsing");
        },
    };
}
