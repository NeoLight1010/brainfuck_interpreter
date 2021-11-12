pub struct Token;

impl Token {
    pub const SHIFT_LEFT: char = '<';
    pub const SHIFT_RIGHT: char = '>';
    pub const INC_DATA: char = '+';
    pub const DEC_DATA: char = '-';
    pub const PRINT_BYTE: char = '.';
    pub const READ_BYTE: char = ',';
    pub const WHILE_START: char = '[';
    pub const WHILE_END: char = ']';
}
