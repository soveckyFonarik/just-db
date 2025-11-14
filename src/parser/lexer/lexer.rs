#[derive(Debug, PartialEq, Clone)]
pub enum LexError {
    InvalidCharacter(char),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    Select,
    From,
    Where,
    Insert,
    Into,
    Values,
    Update,
    Set,
    Delete,
    Create,
    Table,
    Primary,
    Key,
    Foreign,
    References,
    Drop,
    Alter,
    Add,
    Column,
    Constraint,
    Index,
    Join,
    Inner,
    Left,
    Right,
    Full,
    Outer,
    On,
    Group,
    By,
    Order,
    Asc,
    Desc,
    Union,
    All,
    Distinct,
    Limit,
    Offset,
    Having,
    As,
    And,
    Or,
    Not,
    Null,
    Is,
    In,
    Between,
    Like,
    Exists,
    Any,
    Case,
    When,
    Then,
    Else,
    End,
    Default,

    // Data Types
    Int,
    Integer,
    SmallInt,
    TinyInt,
    BigInt,
    Float,
    Real,
    Double,
    Decimal,
    Numeric,
    VarChar,
    Char,
    Text,
    Date,
    Time,
    Timestamp,
    Boolean,

    // Symbols and Operators
    Asterisk,
    Comma,
    Semicolon,
    OpenParen,
    CloseParen,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanOrEquals,
    GreaterThanOrEquals,
    Plus,
    Minus,
    Slash,
    Percent,
    Concat,
    SingleQuote,
    DoubleQuote,

    // Identifiers and Literals
    Identifier(String),
    StringLiteral(String),
    NumericLiteral(String),

    // Comments
    SingleLineComment(String),
    MultiLineComment(String),

    // Whitespace
    Whitespace,

    // EOF
    EOF,
}

#[derive(Debug, Clone)]
struct Lexer<'a> {
    input: &'a str,
    input_iterator: std::str::Chars<'a>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let c = self.skip_whitespace()?;
            // let c = self.input_iterator.next()?;
            // "* , ; ( ) = < >   + - / % |"
            let firstToken = match c {
                // Операторы и разделители
                '*' => return Some(self.single(Token::Asterisk)),
                ',' => return Some(self.single(Token::Comma)),
                '=' => return Some(self.single(Token::Equals)),
                '+' => return Some(self.single(Token::Plus)),
                '-' => return Some(self.single(Token::Minus)),
                '%' => return Some(self.single(Token::Percent)),
                '|' => return Some(self.single(Token::Concat)),
                // single or longer
                '<' => return Some(self.may_by_longer(Token::LessThan)),
                '>' => return Some(self.may_by_longer(Token::GreaterThan)),

                '(' => return Some(Ok(Token::OpenParen)),
                ')' => return Some(Ok(Token::CloseParen)),
                ';' => return Some(Ok(Token::Semicolon)),

                '"' => return Some(self.literal_started(Token::DoubleQuote)),

                '/' => return Some(self.may_by_longer(Token::Slash)),
                '\'' => return Some(self.may_by_longer(Token::SingleQuote)),
                _ => return None,
            };
            // let secondToken =
            // if let Some(token) == firstToken{
            //     return Some(token);
            // }
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            input_iterator: input.chars(),
        }
    }

    pub fn single(&mut self, token: Token) -> Result<Token, LexError> {
        match self.input_iterator.next() {
            Some(c) => {
                if c.is_whitespace() {
                    return Ok(token);
                }
                return Err(LexError::InvalidCharacter(c));
            }
            None => return Ok(token),
        }
    }

    pub fn skip_whitespace(&mut self) -> Option<char> {
        while let Some(c) = self.input_iterator.next() {
            if !c.is_whitespace() {
                return Some(c);
            }
        }
        return None;
    }

    fn may_by_longer(&mut self, first: Token) -> Result<Token, LexError> {
        match first {
            Token::LessThan => match self.input_iterator.next() {
                Some('=') => return Ok(Token::LessThanOrEquals),
                Some('>') => return Ok(Token::NotEquals),
                Some(' ') => return Ok(Token::LessThan),
                Some(c) => return Err(LexError::InvalidCharacter(c)),
                None => return Ok(Token::LessThan),
            },
            Token::GreaterThan => match self.input_iterator.next() {
                Some('=') => return Ok(Token::GreaterThanOrEquals),
                Some(' ') => return Ok(Token::GreaterThan),
                Some(c) => return Err(LexError::InvalidCharacter(c)),
                None => return Ok(Token::GreaterThan),
            },
            Token::Slash => match self.input_iterator.next() {
                Some(' ') => return Ok(Token::Slash),
                Some(c) => return Err(LexError::InvalidCharacter(c)),
                None => return Ok(Token::Slash),
            },
            _ => return Err(LexError::InvalidCharacter('|')),
        }
    }

    fn literal_started(&self, double_quote: Token) -> Result<Token, LexError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::lexer::lexer::{LexError, Lexer, Token};

    #[test]
    fn lex_error() {
        let input = "**";
        let lexer = Lexer::new(input);
        let actual: Vec<Result<Token, LexError>> = lexer.collect();

        let expected = vec![Err(LexError::InvalidCharacter('*'))];
        assert_eq!(actual, expected)
    }

    #[test]
    fn lex_single_chars() {
        let input = "* , ; ( ) = < >   + - / % ";
        let lexer = Lexer::new(input);
        let actual: Vec<Result<Token, LexError>> = lexer.collect();

        let expected = vec![
            Ok(Token::Asterisk),
            Ok(Token::Comma),
            Ok(Token::Semicolon),
            Ok(Token::OpenParen),
            Ok(Token::CloseParen),
            Ok(Token::Equals),
            Ok(Token::LessThan),
            Ok(Token::GreaterThan),
            Ok(Token::Plus),
            Ok(Token::Minus),
            Ok(Token::Slash),
            Ok(Token::Percent),
        ];

        assert_eq!(actual, expected);
    }
}
