mod parse;
mod ast;
mod reduce;


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
            stdout().flush();   //  flush the output nessecery after print!
            match read_line(stdin())? {
                None => panic!(),
                Some(string) if string.trim() == "q" => break,
                Some(code) => {
                    match parse::parse(&code) {
                        Ok(x) => {
                            println!("Parsing successful!");
                            println!("tree: \n{:?}", x);
                            println!("reduced: \n{:?}", reduce(&x));
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
    repl::repl()?;
    Ok(())
}
