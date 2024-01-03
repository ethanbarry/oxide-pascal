/* This file parses Pascal code into a stream of tokens.
 * Tokens are described as an enum, which is defined in
 * this file.
 */
#[derive(Debug)]
pub enum Token {
    /* OPERATORS */
    // Assignment operator:
    Assignment, //  :=
    // Mathematical operators:
    Addition,        //  +  ✓
    Subtraction,     //  -  ✓
    Multiplication,  //  *  ✓
    Exponentiation,  //  **  ✓
    Division,        //  /  ✓
    IntegerDivision, //  div  <----------|- Accept only integer arguments.
    Modulus,         //  mod  or  %   <--|
    PlusEquals,      //  +=  ✓
    MinusEquals,     //  -=  ✓
    TimesEquals,     //  *=  ✓
    DivEquals,       //  /=  ✓
    // Relational operators:
    Equal,        //  =
    NotEqual,     //  <>
    LessThan,     //  <
    GreaterThan,  //  >
    LessEqual,    //  <=
    GreaterEqual, //  >=
    In,           //  in
    // Specific set operators:
    SymmetricDifference, //  ><
    Include,             //  include
    Exclude,             //  exclude
    // Boolean and Bitwise operators:
    Not, //  not
    And, //  and
    Or,  //  or
    Xor, //  xor
    Shl, //  shl  or  <<
    Shr, //  shr  or  >>
    // Class operators:
    Is, //  is
    As, //  as
    /* SYMBOLS */
    Quote,        // '
    OpenBrace,    // {
    CloseBrace,   // }
    OpenParen,    // (
    CloseParen,   // )
    OpenBracket,  // [
    CloseBracket, // ]
    Dot,          // .
    Comma,        // ,
    Colon,        // :
    Caret,        // ^
    AtSign,       // @
    DollarSign,   // $
    HashSign,     // #
    Ampersand,    // &
    /* BASIC KEYWORDS */
    Absolute,
    Array,
    Asm,
    Begin,
    Case,
    Const,
    Constructor,
    Destructor,
    Do,
    Downto,
    Else,
    End,
    File,
    For,
    Function,
    Goto,
    If,
    Implementation,
    Inherited,
    Inline,
    Interface,
    Label,
    Nil,
    Object,
    Of,
    Operator,
    Packed,
    Procedure,
    Program,
    Record,
    Reintroduce,
    Repeat,
    SelfKeyword,
    Set,
    StringKeyword,
    Then,
    To,
    Type,
    Unit,
    Until,
    Uses,
    Var,
    While,
    With,
}

/// Accepts a character, and the one succeeding it,
/// and returns a tuple of (Token, i8) where the
/// Token contains the value of the symbol, and the
/// i8 represents the distance to advance `c` by.
///
/// For example, |'+', '='| returns a tuple of
/// (Token::PlusEquals, 2) because the loop must
/// consume the next char twice.
pub fn char_parse_operator(c: char, next: char) -> (Token, i8) {
    match c {
        '+' => match next {
            // If a `=` follows it was PlusEquals.
            '=' => (Token::PlusEquals, 2),
            _ => (Token::Addition, 1),
        },
        '-' => match next {
            // Same idea.
            '=' => (Token::MinusEquals, 2),
            _ => (Token::Subtraction, 1),
        },
        '*' => match next {
            '=' => (Token::TimesEquals, 2),
            '*' => (Token::Exponentiation, 2),
            _ => (Token::Multiplication, 1),
        },
        '/' => match next {
            '=' => (Token::DivEquals, 2),
            _ => (Token::Division, 1),
        },
        _ => todo!(),
    }
}
