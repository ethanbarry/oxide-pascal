/* This file parses Pascal code into a stream of tokens.
 * Tokens are described as an enum, which is defined in
 * this file.
 */
#[derive(Debug, Clone, Copy)]
pub enum TokenType {
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

pub struct Token {
    token_type: TokenType,
    value: Option<String>,
}

impl Token {
    pub fn get_type(&self) -> TokenType {
        self.token_type
    }

    pub fn get_value(&self) -> Option<String> {
        self.value
    }
}

/// Accepts a character, and the one succeeding it,
/// and returns a tuple of (Token, i8) where the
/// Token contains the value of the symbol, and the
/// i8 represents the distance to advance `c` by.
///
/// For example, |'+', '='| returns a tuple of
/// (Token::PlusEquals, 2) because the loop must
/// consume the next char twice.
// pub fn char_parse_operator(c: char, next: char) -> (Token, i8) {
//     match c {
//         '+' => match next {
//             // If a `=` follows it was PlusEquals.
//             '=' => (Token::PlusEquals, 2),
//             _ => (Token::Addition, 1),
//         },
//         '-' => match next {
//             // Same idea.
//             '=' => (Token::MinusEquals, 2),
//             _ => (Token::Subtraction, 1),
//         },
//         '*' => match next {
//             '=' => (Token::TimesEquals, 2),
//             '*' => (Token::Exponentiation, 2),
//             _ => (Token::Multiplication, 1),
//         },
//         '/' => match next {
//             '=' => (Token::DivEquals, 2),
//             _ => (Token::Division, 1),
//         },
//         _ => todo!(),
//     }
// }

// Later return custom error types. To be used in an iterator map() fn...
pub fn parse(s: &str) -> Result<Token, std::io::Error> {
    // First construct the match statements for all possiblities:
    match s {
        ":=" => Token {
            token_type: TokenType::Assignment,
            value: None,
        },
        _ => todo!(), // Return errors?
    };

    todo!()
}

// Applies the regexes to a string, which is the whole file at once, and returns
// a Vec of the split substrings.
pub fn regexes(s: &str) -> Vec<String> {
    use regex::Regex;

    // Captures the syntax of a normal language okay... Tweak for Pascal!
    let lexer_regex = Regex::new(r#"(?x)(?::=|\+=|\-=|\*=|/=|<=|>=|==|!=|&&|\|\||<<|>>)|(?:\+|-|\*|/|=|<|>|!|&|\||\^|%|~)|(?:\(|\)|\{|\}|\[|\]|\.|,|;|:)|\d+|[a-zA-Z]+"#).unwrap();

    let code_string = "foo: x = 5 + 3 ** (y - 2);";
    for cap in lexer_regex.captures_iter(code_string) {
        println!("{}", cap.get(0).unwrap().as_str());
    }

    let result: Vec<String> = lexer_regex
        .captures_iter(code_string)
        .map(|cap| cap.get(0).unwrap().as_str().to_string())
        .collect();

    dbg!(result);

    todo!()
}

// TODO: Write tests for the regexes/parser functions!
