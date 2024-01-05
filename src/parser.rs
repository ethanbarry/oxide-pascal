/* This file parses Pascal code into a stream of tokens.
 * Tokens are described as an enum, which is defined in
 * this file.
 */
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    /* OPERATORS */
    // Assignment operator:
    Assignment, //  := ✓
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
    Equal,        //  = ✓
    NotEqual,     //  <> ✓
    LessThan,     //  < ✓
    GreaterThan,  //  > ✓
    LessEqual,    //  <= ✓
    GreaterEqual, //  >= ✓
    In,           //  in ✓
    // Specific set operators:
    SymmetricDifference, //  >< ✓
    Include,             //  include ✓
    Exclude,             //  exclude ✓
    // Boolean and Bitwise operators:
    Not, //  not ✓
    And, //  and ✓
    Or,  //  or ✓
    Xor, //  xor ✓
    Shl, //  shl  or  << ✓
    Shr, //  shr  or  >> ✓
    // Class operators:
    Is, //  is ✓
    As, //  as ✓
    /* SYMBOLS */
    Quote,        // ' ✓
    OpenBrace,    // { ✓
    CloseBrace,   // } ✓
    OpenParen,    // ( ✓
    CloseParen,   // ) ✓
    OpenBracket,  // [ ✓
    CloseBracket, // ] ✓
    Semicolon,    // : ✓
    Dot,          // . ✓
    Comma,        // , ✓
    Colon,        // : ✓
    Caret,        // ^ ✓
    AtSign,       // @ ✓
    DollarSign,   // $ ✓
    HashSign,     // # ✓
    Ampersand,    // & ✓
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
    // Identifiers
    Number,
    Identifier,
    // Errors
    Error,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    value: Option<String>,
}

impl Token {
    /// Returns the TokenType associated with this Token.
    pub fn get_type(&self) -> TokenType {
        self.token_type
    }

    /// Returns the optional string value of this Token.
    /// This value is only present in identifiers, numbers,
    /// and errors.
    pub fn get_value(&self) -> Option<String> {
        self.value.to_owned()
    }

    /// Returns true if this Token is of type TokenType::Error.
    pub fn is_error(&self) -> bool {
        if self.token_type == TokenType::Error {
            true
        } else {
            false
        }
    }
}

/// Accepts a single token-type string from regexes() and parses it into a Token.
fn parse(s: &str) -> Token {
    // First construct the match statements for all possiblities:
    match s.to_lowercase().as_str() {
        ":=" => Token {
            token_type: TokenType::Assignment,
            value: None,
        },
        "<=" => Token {
            token_type: TokenType::LessEqual,
            value: None,
        },
        ">=" => Token {
            token_type: TokenType::GreaterEqual,
            value: None,
        },
        "+=" => Token {
            token_type: TokenType::PlusEquals,
            value: None,
        },
        "-=" => Token {
            token_type: TokenType::MinusEquals,
            value: None,
        },
        "*=" => Token {
            token_type: TokenType::TimesEquals,
            value: None,
        },
        "/=" => Token {
            token_type: TokenType::DivEquals,
            value: None,
        },
        "<>" => Token {
            token_type: TokenType::NotEqual,
            value: None,
        },
        "><" => Token {
            token_type: TokenType::SymmetricDifference,
            value: None,
        },
        "**" => Token {
            token_type: TokenType::Exponentiation,
            value: None,
        },
        "<<" | "shl" => Token {
            token_type: TokenType::Shl,
            value: None,
        },
        ">>" | "shr" => Token {
            token_type: TokenType::Shr,
            value: None,
        },
        "=" => Token {
            // Moving on to single-character operators & symbols...
            token_type: TokenType::Equal,
            value: None,
        },
        "+" => Token {
            token_type: TokenType::Addition,
            value: None,
        },
        "-" => Token {
            token_type: TokenType::Subtraction,
            value: None,
        },
        "*" => Token {
            token_type: TokenType::Multiplication,
            value: None,
        },
        "/" => Token {
            token_type: TokenType::Division,
            value: None,
        },
        "%" | "mod" => Token {
            token_type: TokenType::Modulus,
            value: None,
        },
        ">" => Token {
            token_type: TokenType::GreaterThan,
            value: None,
        },
        "<" => Token {
            token_type: TokenType::LessThan,
            value: None,
        },
        "'" => Token {
            token_type: TokenType::Quote,
            value: None,
        },
        "[" | "(." => Token {
            token_type: TokenType::OpenBracket,
            value: None,
        },
        "]" | ".)" => Token {
            token_type: TokenType::CloseBracket,
            value: None,
        },
        "." => Token {
            token_type: TokenType::Dot,
            value: None,
        },
        "," => Token {
            token_type: TokenType::Comma,
            value: None,
        },
        "(" => Token {
            token_type: TokenType::OpenParen,
            value: None,
        },
        ")" => Token {
            token_type: TokenType::CloseParen,
            value: None,
        },
        ":" => Token {
            token_type: TokenType::Colon,
            value: None,
        },
        ";" => Token {
            token_type: TokenType::Semicolon,
            value: None,
        },
        "^" => Token {
            token_type: TokenType::Caret,
            value: None,
        },
        "@" => Token {
            token_type: TokenType::AtSign,
            value: None,
        },
        "{" | "(*" => Token {
            // These tokens both indicate comments.
            token_type: TokenType::OpenBrace,
            value: None,
        },
        "}" | "*)" => Token {
            // Ditto.
            token_type: TokenType::CloseBrace,
            value: None,
        },
        "$" => Token {
            token_type: TokenType::DollarSign,
            value: None,
        },
        "#" => Token {
            token_type: TokenType::HashSign,
            value: None,
        },
        "&" => Token {
            token_type: TokenType::Ampersand,
            value: None,
        },
        // Beginning keywords!
        "in" => Token {
            token_type: TokenType::In,
            value: None,
        },
        "not" => Token {
            token_type: TokenType::Not,
            value: None,
        },
        "and" => Token {
            token_type: TokenType::And,
            value: None,
        },
        "or" => Token {
            token_type: TokenType::Or,
            value: None,
        },
        "xor" => Token {
            token_type: TokenType::Xor,
            value: None,
        },
        "include" => Token {
            token_type: TokenType::Include,
            value: None,
        },
        "exclude" => Token {
            token_type: TokenType::Exclude,
            value: None,
        },
        "is" => Token {
            token_type: TokenType::Is,
            value: None,
        },
        "as" => Token {
            token_type: TokenType::As,
            value: None,
        },
        "absolute" => Token {
            token_type: TokenType::Absolute,
            value: None,
        },
        "array" => Token {
            token_type: TokenType::Array,
            value: None,
        },
        "asm" => Token {
            token_type: TokenType::Asm,
            value: None,
        },
        "begin" => Token {
            token_type: TokenType::Begin,
            value: None,
        },
        "case" => Token {
            token_type: TokenType::Case,
            value: None,
        },
        "const" => Token {
            token_type: TokenType::Const,
            value: None,
        },
        "constructor" => Token {
            token_type: TokenType::Constructor,
            value: None,
        },
        "destructor" => Token {
            token_type: TokenType::Destructor,
            value: None,
        },
        "do" => Token {
            token_type: TokenType::Do,
            value: None,
        },
        "downto" => Token {
            token_type: TokenType::Downto,
            value: None,
        },
        "else" => Token {
            token_type: TokenType::Else,
            value: None,
        },
        "end" => Token {
            token_type: TokenType::End,
            value: None,
        },
        "file" => Token {
            token_type: TokenType::File,
            value: None,
        },
        "for" => Token {
            token_type: TokenType::For,
            value: None,
        },
        "function" => Token {
            token_type: TokenType::Function,
            value: None,
        },
        "goto" => Token {
            token_type: TokenType::Goto,
            value: None,
        },
        "if" => Token {
            token_type: TokenType::If,
            value: None,
        },
        "implementation" => Token {
            token_type: TokenType::Implementation,
            value: None,
        },
        "inherited" => Token {
            token_type: TokenType::Inherited,
            value: None,
        },
        "inline" => Token {
            token_type: TokenType::Inline,
            value: None,
        },
        "interface" => Token {
            token_type: TokenType::Interface,
            value: None,
        },
        "label" => Token {
            token_type: TokenType::Label,
            value: None,
        },
        "nil" => Token {
            token_type: TokenType::Nil,
            value: None,
        },
        "object" => Token {
            token_type: TokenType::Object,
            value: None,
        },
        "of" => Token {
            token_type: TokenType::Of,
            value: None,
        },
        "operator" => Token {
            token_type: TokenType::Operator,
            value: None,
        },
        "packed" => Token {
            token_type: TokenType::Packed,
            value: None,
        },
        "procedure" => Token {
            token_type: TokenType::Procedure,
            value: None,
        },
        "program" => Token {
            token_type: TokenType::Program,
            value: None,
        },
        "record" => Token {
            token_type: TokenType::Record,
            value: None,
        },
        "reintroduce" => Token {
            token_type: TokenType::Reintroduce,
            value: None,
        },
        "repeat" => Token {
            token_type: TokenType::Repeat,
            value: None,
        },
        "self" => Token {
            token_type: TokenType::SelfKeyword,
            value: None,
        },
        "set" => Token {
            token_type: TokenType::Set,
            value: None,
        },
        "string" => Token {
            token_type: TokenType::StringKeyword,
            value: None,
        },
        "then" => Token {
            token_type: TokenType::Then,
            value: None,
        },
        "to" => Token {
            token_type: TokenType::To,
            value: None,
        },
        "type" => Token {
            token_type: TokenType::Type,
            value: None,
        },
        "unit" => Token {
            token_type: TokenType::Unit,
            value: None,
        },
        "until" => Token {
            token_type: TokenType::Until,
            value: None,
        },
        "uses" => Token {
            token_type: TokenType::Uses,
            value: None,
        },
        "var" => Token {
            token_type: TokenType::Var,
            value: None,
        },
        "while" => Token {
            token_type: TokenType::While,
            value: None,
        },
        "with" => Token {
            token_type: TokenType::With,
            value: None,
        },
        // Catch numeric literals.
        s if s.chars().all(|c| char::is_digit(c, 10)) => Token {
            token_type: TokenType::Number,
            value: Some(s.to_owned()),
        },
        // Catch any leftovers, which must be identifiers...
        s if s.chars().all(char::is_alphanumeric) => Token {
            token_type: TokenType::Identifier,
            value: Some(s.to_owned()), // TODO Return the correct case.
        },
        "//" => Token {
            token_type: TokenType::Error,
            value: Some(format!("`{}` is not yet a supported comment format.", s)),
        },
        _ => {
            eprintln!("[ WARN ]: Unknown character(s) `{}`", s);
            Token {
                token_type: TokenType::Error,
                value: Some(s.to_owned()),
            }
        }
    } // End match; return this expression.
}

/// Applies the regexes to a string, which is the whole file at once, and returns
/// a Vec of the matched tokens.
pub fn regexes(s: &str) -> Vec<Token> {
    use regex::Regex;

    // Captures the syntax of a normal language okay... Tweak for Pascal!
    // EDIT: Mostly tweaked; it captures every character on the list, I believe.
    //       Check for anything extraneous that could throw us off...
    let lexer_regex = Regex::new(r#"(?x)(?::=|\+=|\-=|\*=|/=|<=|>=|==|<>|&&|//|\(\*|\*\)|\(\.|\.\)|<<|>>)|(?:\+|-|\*|/|=|<|>|!|&|\||\^|%|~)|(?:\(|\)|\{|\}|\[|\]|\.|,|;|:)|\d+|[a-zA-Z]+"#).unwrap();

    // Testing data; may come in handy:
    // let code_string = "foo: x = 5 + 3 * (y - 2);";
    //                   " ' + - * / = < > [ ] . , ( ) : ^ @ { } $ # & % << >> ** <> >< <= >= := += -= *= /= (* *) (. .) // ";

    // Get a vec of tokens from the regex on the input.
    let syntax_parse: Vec<Token> = lexer_regex
        .captures_iter(s)
        .map(|cap| cap.get(0).unwrap().as_str().to_string())
        .into_iter()
        .map(|s| parse(&s))
        .into_iter()
        .inspect(|token| {
            if token.is_error() {
                match token.get_value() {
                    Some(err_msg) => eprintln!("{}", err_msg),
                    None => eprintln!("[ WARN ]: Error Token with no error message value! This state should be impossible!")
                }
            }
        })
        .collect();

    syntax_parse
}

// TODO: Write tests for the regexes/parser functions!
