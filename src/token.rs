#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    LeftParen, RightParen, LeftSq, RightSq, LeftCurly, RightCurly, LeftAng, RightAng,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star, Caret,

    Ex, ExEq,
    Eq, EqEq,
    Gr, GrEq,
    Ls, LsEq,

    Id, Str, Num,

    And, Struct, Dimension, Unit, For, If, Or, While, Print, Return, Slf, True, False,

    Eof,
}

pub struct Token<'a> {
    t_type: TokenType,
    pub lexeme: &'a str,
    literal: Option<&'a str>,
    line: usize,
    start: usize,
    end: usize,
}

impl <'a> Token <'a> {
    pub fn new(t_type: TokenType, lexeme: &'a str, literal: Option<&'a str>, line: usize, start: usize, end: usize) -> Self {
        Self {
            t_type,
            lexeme,
            literal,
            line,
            start,
            end,
        }
    }

    pub fn to_string(&self) -> String {
        if let Some(l) = self.literal {
            format!("{:?} {} {} {} {} {}", self.t_type, self.lexeme, l, self.line, self.start, self.end)
        } else {
            format!("{:?} {} {} {} {}", self.t_type, self.lexeme, self.line, self.start, self.end)
        }
    }
}