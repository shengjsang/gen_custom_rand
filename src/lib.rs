/// Generate random number or string by custom

pub struct Custom {
    /// length of random number or string
    length: i32,
    charset: CharSet,
}


pub struct  CharSet {
    kind: CharSetKind,
    value: Vec<u8>,
}

pub enum CharSetKind {
    Number,
    Letter,
    Symbol,
    NumberAndLetter,
    NumberAndSymbol,
    LetterAndSymbol,
    NumberLetterAndSymbol,
}

impl Custom {
    pub fn new(length: i32, charset: CharSet) -> Custom {
        Custom {
            length,
            charset,
        }
    }
}
