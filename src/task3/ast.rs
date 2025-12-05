#[derive(Debug, Clone)]
pub enum Ast {
    Var(String),
    Not(Box<Ast>),
    BinOp(BinOp, Box<Ast>, Box<Ast>),
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Or,
    And,
    Xor,
    Equiv,
    Impl,
    Nand,
    Nor,
}

impl BinOp {
    pub fn from_char(c: char) -> Option<BinOp> {
        match c {
            '+' => Some(BinOp::Or),
            '&' => Some(BinOp::And),
            '@' => Some(BinOp::Xor),
            '~' => Some(BinOp::Equiv),
            '>' => Some(BinOp::Impl),
            '|' => Some(BinOp::Nand),
            '!' => Some(BinOp::Nor),
            _ => None,
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            BinOp::Or => "+",
            BinOp::And => "&",
            BinOp::Xor => "@",
            BinOp::Equiv => "~",
            BinOp::Impl => ">",
            BinOp::Nand => "|",
            BinOp::Nor => "!",
        }
    }
}
