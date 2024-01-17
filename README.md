# Lexer

My personal implementation of a [lexer](https://en.wikipedia.org/wiki/Lexical_analysis).

## Principles

The lexer is plugin based.
This is not a [parser](https://en.wikipedia.org/wiki/Parsing) nor a [compiler](https://en.wikipedia.org/wiki/Compiler).

### Tokens

There are 8 premade kinds of token (examples are not mandatory):

| ```TokenKind```   | Explanation                        | Examples                  |
|-------------------|------------------------------------|---------------------------|
| ```KEYWORD```     | Reserved words                     | `if` `return` `...`       |
| ```DELIMITER```   | Paired delimiter symbols           | `()` `[]` `{}` `...`      |
| ```PUNCTUATION``` | Punctuation symbols                | `;` `.` `...`             |
| ```OPERATOR```    | Symbols that operates on arguments | `+` `-` `=` `...`         |
| ```COMMENT```     | Line or block comments             | `//` `/* ... */` `...`    |
| ```WHITESPACE```  | Non-printable characters           | `-`                       |
| ```LITERAL```     | Numerical, logical, textual values | `1` `true` `"true"` `...` |
| ```IDENTIFIER```  | Names assigned in a program        | `x` `temp` `PRINT` `...`  |

These token kinds (except ```IDENTIFIER```) should be constructed with a name that 
can be used to differentiate tokens with same kind.

Each ```TokenKind``` can be associated with one or more ```Pattern``` 
that match them with a ```string``` through a ```Tokenizer```, giving a ```Token```. 

### Lexer

The ```Lexer``` should be constructed with a ```LexerBuilder``` that wraps several ```Tokenizer```.

### Examples

Simple maths ```Lexer```
```rust
let plus = Tokenizer::new(TokenKind::OPERATOR("PLUS"), '+');
let minus = Tokenizer::new(TokenKind::OPERATOR("MINUS"), '-');
let star = Tokenizer::new(TokenKind::OPERATOR("STAR"), '*');
let slash = Tokenizer::new(TokenKind::OPERATOR("SLASH"), '/');
let equal = Tokenizer::new(TokenKind::OPERATOR("EQUAL"), '=');
let number = Tokenizer::new(TokenKind::LITERAL("NUMBER"), |s: &str| {
  let mut dot_seen = false;

  for ch in s.chars() {
    if !ch.is_digit(10) && (ch != '.' || dot_seen) {
      return false;
    } else if ch == '.' {
      dot_seen = true;
    }
  }
  
  true
});
let id_regex = Regex::new(r"[a-zA-Z_$][a-zA-Z_$0-9]*").unwrap();
let id = Tokenizer::new(TokenKind::IDENTIFIER, id_regex);
let whitespace = Tokenizer::new(TokenKind::WHITESPACE("SPACE"), ' ');
let lexer = Lexer::builder()
  .extend(vec![plus, minus, star, slash, equal, number, id, whitespace])
  .build();

lexer.tokenize("x_4 = 2 + 2 = 4 * 0.5")?;
/* [Token { kind: IDENTIFIER, value: "x_4" }, 
  Token { kind: WHITESPACE("SPACE"), value: " " }, 
  Token { kind: OPERATOR("EQUAL"), value: "=" }, 
  Token { kind: WHITESPACE("SPACE"), value: " " }, 
  Token { kind: LITERAL("NUMBER"), value: "2" }, 
  Token { kind: WHITESPACE("SPACE"), value: " " }, 
  Token { kind: OPERATOR("PLUS"), value: "+" }, 
  Token { kind: WHITESPACE("SPACE"), value: " " }, 
  Token { kind: LITERAL("NUMBER"), value: "2" }, 
  Token { kind: WHITESPACE("SPACE"), value: " " }, 
  Token { kind: OPERATOR("EQUAL"), value: "=" }, 
  Token { kind: WHITESPACE("SPACE"), value: " " }, 
  Token { kind: LITERAL("NUMBER"), value: "4" }, 
  Token { kind: WHITESPACE("SPACE"), value: " " }, 
  Token { kind: OPERATOR("STAR"), value: "*" }, 
  Token { kind: WHITESPACE("SPACE"), value: " " }, 
  Token { kind: LITERAL("NUMBER"), value: "0.5" }] */
```
