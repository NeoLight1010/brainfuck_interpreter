mod token;
mod data;

use std::io;
use data::Data;
use token::Token;


fn main() -> io::Result<()> {
    let mut buffer = String::new();

    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    interpret(&buffer);

    Ok(())
}

fn interpret(code: &String) {
    let mut data = Data::new();

    for c in code.chars() {
        match c {
            Token::SHIFT_LEFT => {
                data.dec_pointer();
            }

            Token::SHIFT_RIGHT => {
                data.inc_pointer();
            }

            Token::INC_DATA => {
                data.inc_cell();
            },

            Token::DEC_DATA => { data.dec_cell() }

            Token::PRINT_BYTE => {
                println!("{}", data.get_cell());
            }

            Token::READ_BYTE => {
                let stdin = io::stdin();
                let mut buffer = String::new();

                stdin.read_line(&mut buffer).unwrap();

                data.set_cell(buffer.chars().nth(0).unwrap() as u32);
            }

            Token::WHILE_START => {}

            Token::WHILE_END => {}

            '\n' => {}

            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {}
