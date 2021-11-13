mod token;
mod data;
mod bf_code;

use std::io;
use bf_code::BFCode;
use data::Data;
use token::Token;


fn main() -> Result<(), String> {
    let mut buffer: String = String::new();

    let stdin = io::stdin();
    match stdin.read_line(&mut buffer) {
        Ok(_) => {}
        Err(_) => {
            return Err("Error reading from stdin.".to_string());
        }
    }

    let code = analyze(&buffer)?;

    interpret(&code);

    Ok(())
}

fn analyze(text: &String) -> Result<BFCode, String> {
    let mut bf_code = BFCode::new(text);
    let mut bracket_stack: Vec<usize> = vec![];

    for (i, c) in bf_code.text.chars().enumerate() {
        match c {
            Token::WHILE_START => {
                bracket_stack.push(i);
            }

            Token::WHILE_END => {
                let start = bracket_stack.pop();

                match start {
                    Some(s) => {
                        bf_code.loops.insert(s, i);
                    }

                    None => {
                        return Err("Unmatched ']'.".to_string());
                    }
                }
            }

            _ => {}
        }
    }

    if !bracket_stack.is_empty() {
        return Err("Unmatched '['.".to_string());
    }

    Ok(bf_code)
}

fn interpret(code: &BFCode) {
    let mut data = Data::new();
    let mut cursor: usize = 0;

    while cursor < code.text.len() {
        let c = code.text.chars().nth(cursor).unwrap();

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
                print!("{}", char::from_u32(data.get_cell()).unwrap());
            }

            Token::READ_BYTE => {
                let stdin = io::stdin();
                let mut buffer = String::new();

                stdin.read_line(&mut buffer).unwrap();

                data.set_cell(buffer.chars().nth(0).unwrap() as u32);
            }

            Token::WHILE_START => {
                if data.get_cell() == 0 {
                    cursor = *code.loops.get_by_left(&cursor).unwrap();
                }
            }

            Token::WHILE_END => {
                if data.get_cell() != 0 {
                    cursor = *code.loops.get_by_right(&cursor).unwrap();
                }
            }

            '\n' => {}

            _ => {}
        }

        cursor += 1;
    }
}

#[cfg(test)]
mod tests {}
