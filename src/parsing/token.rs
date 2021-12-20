#[derive(Debug)]
pub enum Token {
    INT(i32),
    FLOAT(f32),
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    LBRACK,
    RBRACK,
}

