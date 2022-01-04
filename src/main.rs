mod parse;
mod ast;
mod reduce;
mod pretty;
mod r#type;

use std::io::{self, Read};
use ast::*;

mod script {
    use super::*;
    use std::fs::File;

    pub fn run(mut file: File) -> io::Result<()> {
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        dbg!(&buf);
        match parse::parse(&buf) {
            Err(e) => {
                eprintln!("Parsing error: {:?}", e);
            }
            Ok(ast) => {
                let result = reduce::reduce_or_ret(std::rc::Rc::new(ast));
                println!("{}", result);
            }
        }
        Ok(())
    }
}


mod repl {
    use std::io::*;
    use super::*;
    use crate::reduce::reduce;

    fn read_line(stream: Stdin) -> Result<Option<String>> {
        let mut buf = String::new();
        Ok(if let 0 = stream.read_line(&mut buf)? { None } else {
            if buf.ends_with("\r\n") { buf.truncate(buf.len() - 2); }
            else if buf.ends_with("\n") || buf.ends_with("\r") { buf.truncate(buf.len() - 1); }
            Some(buf)
        })
    }

    pub fn repl() -> std::io::Result<()> {
        loop {
            print!("> ");
            stdout().flush()?;   //  flush the output nessecery after print!
            match read_line(stdin())? {
                None => panic!(),
                Some(string) if string.trim() == "q" => break,
                Some(code) => {
                    match parse::parse(&code) {
                        Ok(x) => {
                            println!("Parsing successful!");
                            println!("tree: \n{}", x);
                            match r#type::Type::of_expr(&x, &r#type::VariableTypes::new()) {
                                Some(t) => println!("type: \n{}", t),
                                None => println!("Type checker failed"),
                            }
                            match reduce(&x) {
                                Some(x) => println!("reduced: \n{}", x),
                                None => println!("could not reduce"),
                            };
                        },
                        Err(()) => {
                            eprintln!("Failed parsing");
                        },
                    };
                },
            }
        };
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    use clap::{App, Arg};
    let arg_matches =
        App::new("Expression Script")
        .author("Yonatan R. <yony252525@gmail.com>")
        .about("Run Expression Script programs and repls")
        .arg(Arg::with_name("INPUT"))
        .get_matches();

    if let Some(filename) = arg_matches.value_of("INPUT") {
        let file = std::fs::File::open(filename)?;
        script::run(file)?;
    } else {
        repl::repl()?;
    }
    Ok(())
}
