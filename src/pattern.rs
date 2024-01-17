/*!
Module for Pattern matching. \

A Pattern matching API which provides a generic trait for using different pattern types when searching through a `&str`. \
For more details on implementation, see the trait [`Pattern`].

# Examples

[`Pattern`] is implemented for `char`, `&str`, slices of `char` and `&str`, [`Regex`](https://docs.rs/regex/latest/regex/struct.Regex.html)
and closures implementing `Fn(&str) -> bool`.

```
# use plexer::pattern::Pattern;
#
let hay = "Can you find a needle in a haystack";

// char pattern
assert!('n'.find_one_in(hay).is_some_and(|m| m.start == 2));
// &str pattern
assert!("you".find_one_in(hay).is_some_and(|m| m.start == 4));
// array of chars pattern
assert!(['a', 'e', 'i', 'o', 'u'].find_one_in(hay).is_some_and(|m| m.start == 1));
// array of &str pattern
assert!(["Can", "you"].find_one_in(hay).is_some_and(|m| m.start == 0));
// closure pattern
assert!((|s: &str| s.starts_with("f")).find_one_in(hay).is_some_and(|m| m.start == 8));
```
*/

use regex::Regex;

/// Returned by [`Pattern`] on match.
#[derive(Debug, Clone, PartialEq)]
pub struct Match<'a> {
    /// The string that was searched in
    pub haystack: &'a str,
    /// Start of the match
    pub start: usize,
    /// End of the match
    pub end: usize,
}

impl<'a> Match<'a> {
    /**
    Create a match from a haystack `&str` and `start..end` range.

    # Panics
    When ```start >= end``` or ```haystack.len() < end```.

    # Example
    ```should_panic
    # use plexer::pattern::Match;
    #
    let mat = Match::new("don't go to far...", 0, 100000);
    ```
    */
    pub fn new(haystack: &'a str, start: usize, end: usize) -> Self {
        assert!(start < end);
        assert!(haystack.len() >= end);
        Self {
            haystack,
            start,
            end,
        }
    }

    /**
    Returns the number of char in the match

    # Example
    ```
    # use plexer::pattern::Match;
    #
    assert_eq!(Match::new("three", 1, 4).len(), 3);
    ```
    */
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /**
    Convert to to `&str`.

    # Example
    ```
    # use plexer::pattern::Match;
    #
    let mat = Match::new("it's here not here", 5, 9);

    assert_eq!(mat.as_str(), "here");
    ```
    */
    pub fn as_str(&self) -> &'a str {
        &self.haystack[self.start..self.end]
    }
}

impl<'a> ToString for Match<'a> {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

/**
A string `Pattern` trait.

The type implementing it can be used as a pattern for `&str`,
by default it is implemented for the following types.

| Pattern type              | Match condition                         |
|---------------------------|-----------------------------------------|
| ```char```                | is contained in string                  |
| ```&str```                | is substring                            |
| ```String```              | is substring                            |
| ```&[char]```             | any `char` match                        |
| ```&[&str]```             | any `&str` match                        |
| ```F: Fn(&str) -> bool``` | `F` returns `true` for substring        |
| ```Regex```               | `Regex` match substring                 |
*/
pub trait Pattern<'a> {
    /**
    Find all occurences of the pattern in the given `&str`.

    # Examples
    ```
    # use plexer::pattern::{Match, Pattern};
    #
    assert!("ab".find_in("cd").is_empty());
    assert_eq!("ab".find_in("cabd"), vec![Match::new("cabd", 1, 3)]);
    ```
    */
    fn find_in(&self, haystack: &'a str) -> Vec<Match<'a>>;

    /**
    Find all occurences of the pattern in the given `&str` that are prefixes.

    # Examples
    ```
    # use plexer::pattern::{Match, Pattern};
    #
    assert!("ab".find_prefixes_in("cdab").is_empty());
    assert_eq!("ab".find_prefixes_in("abcd"), vec![Match::new("abcd", 0, 2)]);
    ```
    */
    fn find_prefixes_in(&self, haystack: &'a str) -> Vec<Match<'a>> {
        self.find_in(haystack)
            .into_iter()
            .filter(|mat| mat.start == 0)
            .collect()
    }

    /**
    Find all occurences of the pattern in the given `&str` that are suffixes.

    # Examples
    ```
    # use plexer::pattern::{Match, Pattern};
    #
    assert!("ab".find_suffixes_in("abcd").is_empty());
    assert_eq!("ab".find_suffixes_in("cdab"), vec![Match::new("cdab", 2, 4)]);
    ```
    */
    fn find_suffixes_in(&self, haystack: &'a str) -> Vec<Match<'a>> {
        let len = haystack.len();
        self.find_in(haystack)
            .into_iter()
            .filter(|mat| mat.end == len)
            .collect()
    }

    /**
    Find one occurrence of the pattern in the given `&str`.

    # Examples
    ```
    # use plexer::pattern::{Match, Pattern};
    #
    assert!("ab".find_one_in("cd").is_none());
    assert_eq!("ab".find_one_in("cdab"), Some(Match::new("cdab", 2, 4)));
    ```
    */
    fn find_one_in(&self, haystack: &'a str) -> Option<Match<'a>> {
        self.find_in(haystack).into_iter().next()
    }

    /**
    Find one occurrence of the pattern in the given `&str` that is prefix.

    # Examples
    ```
    # use plexer::pattern::{Match, Pattern};
    #
    assert!("ab".find_prefix_in("cdab").is_none());
    assert_eq!("ab".find_prefix_in("abcd"), Some(Match::new("abcd", 0, 2)));
    ```
    */
    fn find_prefix_in(&self, haystack: &'a str) -> Option<Match<'a>> {
        self.find_prefixes_in(haystack).into_iter().next()
    }

    /**
    Find one occurrence of the pattern in the given `&str` that is suffix.

    # Examples
    ```
    # use plexer::pattern::{Match, Pattern};
    #
    assert!("ab".find_suffix_in("abcd").is_none());
    assert_eq!("ab".find_suffix_in("cdab"), Some(Match::new("cdab", 2, 4)));
    ```
    */
    fn find_suffix_in(&self, haystack: &'a str) -> Option<Match<'a>> {
        self.find_suffixes_in(haystack).into_iter().next()
    }
}

impl<'a> Pattern<'a> for char {
    fn find_in(&self, haystack: &'a str) -> Vec<Match<'a>> {
        haystack
            .match_indices(&self.to_string())
            .map(|(i, mat)| Match::new(haystack, i, i + mat.len()))
            .collect()
    }
}

impl<'a> Pattern<'a> for [char] {
    fn find_in(&self, haystack: &'a str) -> Vec<Match<'a>> {
        self.iter().flat_map(|ch| ch.find_in(haystack)).collect()
    }
}

impl<'a, const N: usize> Pattern<'a> for [char; N] {
    fn find_in(&self, haystack: &'a str) -> Vec<Match<'a>> {
        self.as_slice().find_in(haystack)
    }
}

impl<'a, const N: usize> Pattern<'a> for &[char; N] {
    fn find_in(&self, haystack: &'a str) -> Vec<Match<'a>> {
        self.as_slice().find_in(haystack)
    }
}

impl<'a> Pattern<'a> for String {
    fn find_in(&self, haystack: &'a str) -> Vec<Match<'a>> {
        haystack
            .match_indices(self)
            .map(|(i, mat)| Match::new(haystack, i, i + mat.len()))
            .collect()
    }
}

impl<'a> Pattern<'a> for &str {
    fn find_in(&self, haystack: &'a str) -> Vec<Match<'a>> {
        self.to_string().find_in(haystack)
    }
}

impl<'a> Pattern<'a> for [&str] {
    fn find_in(&self, haystack: &'a str) -> Vec<Match<'a>> {
        self.iter().flat_map(|ch| ch.find_in(haystack)).collect()
    }
}

impl<'a, const N: usize> Pattern<'a> for [&str; N] {
    fn find_in(&self, haystack: &'a str) -> Vec<Match<'a>> {
        self.as_slice().find_in(haystack)
    }
}

impl<'a, const N: usize> Pattern<'a> for &[&str; N] {
    fn find_in(&self, haystack: &'a str) -> Vec<Match<'a>> {
        self.as_slice().find_in(haystack)
    }
}

impl<'a: 'b, 'b, F> Pattern<'a> for F
where
    F: Fn(&'b str) -> bool,
{
    fn find_in(&self, haystack: &'a str) -> Vec<Match<'a>> {
        let mut matches = Vec::new();
        let mut cur_1 = 0;
        // The goal is to check from left to right and to take the largest match
        while cur_1 < haystack.len() {
            let mut cur_2 = haystack.len();
            while cur_2 > cur_1 {
                let sub = &haystack[cur_1..cur_2];
                if (self)(sub) {
                    matches.push(Match::new(haystack, cur_1, cur_2));
                    cur_1 = cur_2;
                }
                cur_2 -= 1
            }
            cur_1 += 1;
        }
        matches
    }
}

impl<'a> Pattern<'a> for Regex {
    fn find_in(&self, haystack: &'a str) -> Vec<Match<'a>> {
        self.find_iter(haystack)
            .map(|mat| Match::new(haystack, mat.start(), mat.end()))
            .collect()
    }
}
