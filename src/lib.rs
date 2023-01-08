/// Generate random number or string by custom

pub struct Custom {
    /// length of random number or string
    length: usize,
    charset: CharSet,
}


pub struct CharSet {
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
    pub fn new(length: usize, charset: CharSet) -> Custom {
        Custom {
            length,
            charset,
        }
    }

    pub fn generate(&self) -> Vec<u8> {
        let mut charset: Vec<u8> = Vec::new();
        match self.charset.kind {
            CharSetKind::Number => {
                charset = b"0123456789".to_vec();
            }
            CharSetKind::Letter => {
                charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            ".to_vec();
            }
            CharSetKind::Symbol => {
                charset = b"~!@#$%^&*()_+-={}[]|:;<>,.?/\"\\".to_vec();
            }
            CharSetKind::NumberAndLetter => {
                charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789".to_vec();
            }
            CharSetKind::NumberAndSymbol => {
                charset = b"0123456789~!@#$%^&*()_+-={}[]|:;<>,.?/\"\\".to_vec();
            }
            CharSetKind::LetterAndSymbol => {
                charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                           ~!@#$%^&*()_+-={}[]|:;<>,.?/\"\\".to_vec();
            }
            CharSetKind::NumberLetterAndSymbol => {
                charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789~!@#$%^&*()_+-={}[]|:;<>,.?/\"\\".to_vec();
            }
        }
        charset
    }
}
