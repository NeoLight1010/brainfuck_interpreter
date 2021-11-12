mod token;

use std::{collections::HashMap, convert::TryInto, io};
use token::Token;


fn main() -> io::Result<()> {
    let mut buffer = String::new();

    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    interpret(&buffer);

    Ok(())
}

fn interpret(code: &String) {
    let mut data: HashMap<i32, i32> = HashMap::new();
    let mut pointer = 0;

    for c in code.chars() {
        match c {
            Token::SHIFT_LEFT => {
                pointer -= 1;
            }

            Token::SHIFT_RIGHT => {
                pointer += 1;
            }

            Token::INC_DATA => match data.get_mut(&pointer) {
                Some(v) => {
                    *v += 1;
                }

                None => {
                    data.insert(pointer, 1);
                }
            },

            Token::DEC_DATA => match data.get_mut(&pointer) {
                Some(v) => {
                    *v -= 1;
                }

                None => {
                    data.insert(pointer, -1);
                }
            },

            Token::PRINT_BYTE => {
                match data.get(&pointer) {
                    Some(_) => {}

                    None => {
                        data.insert(pointer, 0);
                    }
                }

                println!("{}", data.get(&pointer).unwrap());
            }

            Token::READ_BYTE => {
                let stdin = io::stdin();
                let mut buffer = String::new();

                stdin.read_line(&mut buffer).unwrap();

                data.insert(
                    pointer,
                    (buffer.chars().nth(0).unwrap() as u32).try_into().unwrap(),
                );
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
