# Pattern Lexer

My personal implementation of a [lexer](https://en.wikipedia.org/wiki/Lexical_analysis).

## Principle

This lexer is making use of the `Pattern` trait to find tokens. \
The idea is to create `Tokens`, explain how to match them with a `Pattern` and build them from the matched `String` value.

### Pattern

A string `Pattern` trait.

The type implementing it can be used as a pattern for `&str`,
by default it is implemented for the following types.

| Pattern type          | Match condition                         |
|-----------------------|-----------------------------------------|
| `char`                | is contained in string                  |
| `&str`                | is substring                            |
| `String`              | is substring                            |
| `&[char]`             | any `char` match                        |
| `&[&str]`             | any `&str` match                        |
| `F: Fn(&str) -> bool` | `F` returns `true` for substring        |
| `Regex`               | `Regex` match substring                 |

### Usage

The `lexer!` macro match the following syntax.

```rust 
lexer!(
    // Ordered by priority
    NAME(optional types, ...) {
        impl Pattern => |value: String| -> Token,
        ...,
    },
    ...,
);
```

It generates module `gen` which contains `Token`, `LexerError`, `LexerResult` and `Lexer`.

You can now call `Lexer::tokenize` to tokenize a `&str`,
it should returns a `Lexer` instance that implements `Iterator`. \
Each iteration, the `Lexer` try to match one of the given `Pattern` and return a `LexerResult<Token>` built from the best match.

### Example

Here is an example for a simple math lexer.

```rust
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

```rust
mod lexer {
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

```rust
let mut lex = gen::Lexer::tokenize("x_4 = 1 + 3 = 2 * 2");
assert_eq!(lex.nth(2), Some(Ok(gen::Token::OPERATOR('='))));
assert_eq!(lex.nth(5), Some(Ok(gen::Token::NUMBER(3))));
// Our lexer doesn't handle parenthesis...
let mut err = gen::Lexer::tokenize("x_4 = (1 + 3)");
assert!(err.nth(4).is_some_and(|res| res.is_err()));
```
