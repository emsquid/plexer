/*!
**P**attern matching **LEXER**[^note] implementation.

[^note]: More details on [Lexical analysis](https://en.wikipedia.org/wiki/Lexical_analysis).

# Principle
This lexer is making use of the [`Pattern`](pattern::Pattern) trait to find tokens. \
The idea is to create `Tokens`, explain how to match them with a `Pattern` and build them from the matched `String` value.

```ignore
lexer!(
    // Ordered by priority
    NAME(optional types, ...) {
        impl Pattern => |value: String| -> Token,
        ...,
    },
    ...,
);
```

The [`lexer!`] macro generates module `gen` which contains `Token`, `LexerError`, `LexerResult` and `Lexer`.

You can now call `Lexer::tokenize` to tokenize a `&str`,
it should returns a `Lexer` instance that implements `Iterator`. \
Each iteration, the `Lexer` try to match one of the given `Pattern` and return a `LexerResult<Token>` built from the best match.

# Example
Here is an example for a simple math lexer.
```
# use regex::Regex;
# use plexer::lexer;
#
lexer!(
    // Different operators
    OPERATOR(char) {
        '+' => |_| Token::OPERATOR('+'),
        '-' => |_| Token::OPERATOR('-'),
        '*' => |_| Token::OPERATOR('*'),
        '/' => |_| Token::OPERATOR('/'),
        '=' => |_| Token::OPERATOR('='),
    },
    // Integer numbers
    NUMBER(usize) {
        |s: &str| s.chars().all(|c| c.is_digit(10))
            => |v: String| Token::NUMBER(v.parse().unwrap()),
    },
    // Variable names
    IDENTIFIER(String) {
        regex!(r"[a-zA-Z_$][a-zA-Z_$0-9]*")
            => |v: String| Token::IDENTIFIER(v),
    },
    WHITESPACE {
        [' ', '\n'] => |_| Token::WHITESPACE,
    },
);
```
That will expand to these enum and structs.
```ignore
mod gen {
    pub enum Token {
        OPERATOR(char),
        NUMBER(usize),
        IDENTIFIER(String),
        WHITESPACE,
    }
    pub struct Lexer {...}
    pub struct LexerError {...}
    pub type LexerResult<T> = Result<T, LexerError>;
    // ...
}
```
And you can use them afterwards.
```
# use plexer::lexer;
#
# lexer!(
#     OPERATOR(char) {
#         '+' => |_| Token::OPERATOR('+'),
#         '-' => |_| Token::OPERATOR('-'),
#         '*' => |_| Token::OPERATOR('*'),
#         '/' => |_| Token::OPERATOR('/'),
#         '=' => |_| Token::OPERATOR('='),
#     },
#     NUMBER(usize) {
#         |s: &str| s.chars().all(|c| c.is_digit(10))
#             => |v: String| Token::NUMBER(v.parse().unwrap()),
#     },
#     IDENTIFIER(String) {
#         regex!(r"[a-zA-Z_$][a-zA-Z_$0-9]*")
#             => |v: String| Token::IDENTIFIER(v),
#     },
#     WHITESPACE {
#         [' ', '\n'] => |_| Token::WHITESPACE,
#     },
# );
let mut lex = gen::Lexer::tokenize("x_4 = 1 + 3 = 2 * 2");
assert_eq!(lex.nth(2), Some(Ok(gen::Token::OPERATOR('='))));
assert_eq!(lex.nth(5), Some(Ok(gen::Token::NUMBER(3))));
// Our lexer doesn't handle parenthesis...
let mut err = gen::Lexer::tokenize("x_4 = (1 + 3)");
assert!(err.nth(4).is_some_and(|res| res.is_err()));
```
*/

pub mod pattern;

/**
Macro to build a [Regex](https://docs.rs/regex/latest/regex/struct.Regex.html).

# Panics
If the given pattern is not `@safe` and not a valid regex.
```should_panic
# use plexer::regex;
#
let err = regex!("(");
```

# Example
```
# use plexer::regex;
#
// Unwrap inside the macro
let re = regex!("t|e|s|t");
// Don't unwrap
let gex = regex!(@safe "t|e|s|t").unwrap();
```
**/
#[macro_export]
macro_rules! regex {
    ($pattern:literal) => {
        regex::Regex::new($pattern).unwrap()
    };
    (@safe $pattern:literal) => {
        regex::Regex::new($pattern)
    };
}

/**
Macro to build your own plugin-based lexer.

# Usage
```ignore
lexer!(
    // Ordered by priority
    NAME(optional types, ...) {
        impl Pattern => |value: String| -> Token,
        ...,
    },
    ...,
);
```

# Example
Here is an example for a simple condition statement lexer.
```
# use plexer::lexer;
#
lexer!(
    DELIMITER(char) {
        '{' => |_| Token::DELIMITER('{'),
        '}' => |_| Token::DELIMITER('}'),
    },
    KEYWORD(String) {
        "if" => |v: String| Token::KEYWORD(v),
        "else" => |v: String| Token::KEYWORD(v),
    },
    IDENTIFIER(String) {
        regex!(r"[a-zA-Z_$][a-zA-Z_$0-9]*")
            => |v: String| Token::IDENTIFIER(v),
    },
    WHITESPACE {
        [' ', '\n', '\t'] => |_| Token::WHITESPACE,
    },
);

let mut lex = gen::Lexer::tokenize("if test { one } else { two }");
assert_eq!(lex.next(), Some(Ok(gen::Token::KEYWORD(String::from("if")))));
```
**/
#[macro_export]
macro_rules! lexer {
    ($($token:tt $(($($field: ty),+))? {$( $pattern:expr => $build:expr,)+}),* $(,)*) => {
        mod gen {
            use $crate::regex;
            use $crate::pattern::Pattern;

            #[derive(Debug, Clone, PartialEq)]
            pub enum Token {
                $($token$(($($field),+))?),*
            }

            #[derive(Debug, Clone, PartialEq)]
            pub struct LexerError<'a> {
                haystack: &'a str,
                cursor: usize,
            }

            impl<'a> LexerError<'a> {
                 fn new(haystack: &'a str, cursor:usize) -> Self {
                     Self { haystack, cursor }
                 }
            }

            impl<'a> std::fmt::Display for LexerError<'a> {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        write!(
                            f, "unexpected character '{}' at index {}",
                            &self.haystack[self.cursor..=self.cursor],
                            self.cursor
                        )
                }
            }

            pub type LexerResult<'a, T> = Result<T, LexerError<'a>>;

            #[derive(Debug)]
            pub struct Lexer<'a> {
                haystack: &'a str,
                cursor: usize,
            }

            #[allow(dead_code)]
            impl<'a> Lexer<'a> {
                pub fn tokenize(haystack: &'a str) -> Self {
                    Self { haystack, cursor: 0 }
                }
            }

            impl<'a> Iterator for Lexer<'a> {
                type Item = LexerResult<'a, Token>;

                fn next(&mut self) -> Option<Self::Item> {
                    if self.cursor < self.haystack.len() {
                        let mut token = None;
                        let mut len = 0;

                        $($({
                            if let Some(mat) = $pattern.find_prefix_in(&self.haystack[self.cursor..]) {
                                if mat.len() > len {
                                    token = Some($build(mat.to_string()));
                                    len = mat.len();
                                }
                            }
                        })+)*

                        self.cursor += std::cmp::max(len, 1);
                        Some(token.ok_or(LexerError::new(self.haystack.clone(), self.cursor - 1)))
                    } else {
                        None
                    }
                }
            }
        }
    };
}
