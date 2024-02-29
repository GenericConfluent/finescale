use enumflags2::bitflags;
use std::iter::Enumerate;
use std::str::Chars;
use tinystr::TinyAsciiStr;

// The largest department code is 6 ascii chars long.
pub type DepartmentCode = TinyAsciiStr<6>;

pub struct Lexer<'a> {
    source: &'a str,
    cursor: Enumerate<Chars<'a>>,
}

#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RequirementType {
    Prerequisite,
    Corequisite,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Type(RequirementType),
    Delimiter(char),
    Logical(LogicalOperator),
    Department(DepartmentCode),
    Number(usize),
}

impl Lexer<'_> {
    fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) -> usize {
        loop {
            if let Some((idx, ch)) = self.cursor.clone().next() {
                if predicate(ch) {
                    self.cursor.next();
                    continue;
                }
                return idx;
            }
            return self.source.len();
        }
    }
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(source: &'a str) -> Self {
        Self {
            source,
            cursor: source.chars().enumerate(),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (begin, ch) = self.cursor.next()?;
            if ch.is_whitespace() {
                continue;
            }
            return Some(match ch {
                delim if [':', ',', '.', ';'].contains(&delim) => Token::Delimiter(delim),
                'a'..='z' | 'A'..='Z' => {
                    let end = self.eat_while(|ch| ch.is_ascii_alphabetic());
                    let raw_token = &self.source[begin..end];
                    match raw_token {
                        "and" | "And" => Token::Logical(LogicalOperator::And),
                        "or" | "Or" => Token::Logical(LogicalOperator::Or),
                        prereq if prereq.to_ascii_lowercase().starts_with("prerequisite") => {
                            Token::Type(RequirementType::Prerequisite)
                        }
                        coreq
                            if ["co-requisite", "corequisite"]
                                .iter()
                                .any(|prefix| coreq.to_ascii_lowercase().starts_with(prefix)) =>
                        {
                            Token::Type(RequirementType::Corequisite)
                        }
                        department
                            if (2..=6).contains(&department.len())
                                && department.chars().all(|ch| {
                                    ch.is_ascii_alphabetic() && ch.is_ascii_uppercase()
                                }) =>
                        {
                            Token::Department(TinyAsciiStr::from_str(department).expect(
                                "Known to have max 6 chars and that those chars are alphabetic \
                                 ascii",
                            ))
                        }
                        _ => continue,
                    }
                }
                '0'..='9' => {
                    let end = self.eat_while(|ch| ch.is_ascii_digit());
                    Token::Number(
                        self.source[begin..end]
                            .parse()
                            .expect("Should only contain ascii digits"),
                    )
                }
                _ => continue,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lex_delimiters() {
        let lexer = Lexer::from(": {}{}[];  +sdfas.d,");
        let expected = vec![
            Token::Delimiter(':'),
            Token::Delimiter(';'),
            Token::Delimiter('.'),
            Token::Delimiter(','),
        ];
        assert_eq!(lexer.collect::<Vec<_>>(), expected);
    }

    #[test]
    fn test_lex_realistic() {
        let input = "Animal behavior from an ethological perspective, with
emphasis on the mechanisms underlying a variety of behaviors. The material is
intended to complement that of ZOOL 371. Prerequisite or corequisite: ZOOL 241
or 242 or PHYSL 210, or 212 or 214. Offered in alternate years.";
        let lexer = Lexer::from(input);
        let expected = vec![
            Token::Delimiter(','),
            Token::Delimiter('.'),
            Token::Department(TinyAsciiStr::from_str("ZOOL").unwrap()),
            Token::Number(371),
            Token::Delimiter('.'),
            Token::Type(RequirementType::Prerequisite),
            Token::Logical(LogicalOperator::Or),
            Token::Type(RequirementType::Corequisite),
            Token::Delimiter(':'),
            Token::Department(TinyAsciiStr::from_str("ZOOL").unwrap()),
            Token::Number(241),
            Token::Logical(LogicalOperator::Or),
            Token::Number(242),
            Token::Logical(LogicalOperator::Or),
            Token::Department(TinyAsciiStr::from_str("PHYSL").unwrap()),
            Token::Number(210),
            Token::Delimiter(','),
            Token::Logical(LogicalOperator::Or),
            Token::Number(212),
            Token::Logical(LogicalOperator::Or),
            Token::Number(214),
            Token::Delimiter('.'),
            Token::Delimiter('.'),
        ];
        assert_eq!(lexer.collect::<Vec<_>>(), expected);
    }
}
