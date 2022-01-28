pub struct Position {
    pub line: u32,
    pub character: usize
}

pub struct IllegalCharacterError {
    pub pos: Position,
    pub cause: char,
}

trait Error {
    fn make_msg(&self) -> String;
}

impl IllegalCharacterError {
    pub fn new(pos: Position, chara: char) -> IllegalCharacterError {
        IllegalCharacterError {
            pos,
            cause: chara
        }
    }
}

impl Error for IllegalCharacterError {
    fn make_msg(&self) -> String {
        String::from(format!("IllegalCharacterError at {}, {}: Unknown character '{}'", self.pos.line, self.pos.character % self.pos.line as usize, self.cause))
    }
}
