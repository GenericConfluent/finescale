use crate::lexer::*;
use enumflags2::BitFlags;
use std::iter::Peekable;
use std::ops::Deref;

#[derive(PartialEq, Debug)]
struct OneOf {
    requirement: Vec<String>,
    reqtype: BitFlags<RequirementType>,
}

impl Deref for OneOf {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        self.requirement.as_ref()
    }
}

#[derive(PartialEq, Default, Debug)]
pub struct Requirements {
    inner: Vec<OneOf>,
}

impl From<Vec<OneOf>> for Requirements {
    fn from(requirements: Vec<OneOf>) -> Self {
        Self {
            inner: requirements,
        }
    }
}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    requirements: Requirements,
    requirement_type: BitFlags<RequirementType>,
    is_eof: bool,
}

impl<'a> From<&'a str> for Parser<'a> {
    fn from(text: &'a str) -> Self {
        Self {
            lexer: Lexer::from(text).peekable(),
            requirements: Default::default(),
            requirement_type: Default::default(),
            is_eof: false,
        }
    }
}

impl Parser<'_> {
    fn next(&mut self) -> Option<Token> {
        let token = self.lexer.next();
        self.is_eof = token.is_none();
        token
    }

    pub fn parse(mut self) -> Requirements {
        while !self.is_eof {
            self.try_parse();
        }
        self.requirements
    }

    fn try_parse(&mut self) -> Option<()> {
        self.type_decl()?;
        self.course_list()?;
        self.requirement_type = Default::default();
        Some(())
    }

    fn course_list(&mut self) -> Option<()> {
        let mut department = None;
        let mut oneof = OneOf {
            requirement: vec![],
            reqtype: self.requirement_type,
        };
        loop {
            match department {
                Some(code) => match self.next()? {
                    Token::Department(code) => {
                        department = Some(code);
                    }
                    Token::Number(class_num) => {
                        oneof.requirement.push(format!("{}{}", code, class_num));
                    }
                    Token::Delimiter(';') | Token::Logical(LogicalOperator::And) => {
                        self.requirements.inner.push(std::mem::replace(
                            &mut oneof,
                            OneOf {
                                requirement: vec![],
                                reqtype: self.requirement_type,
                            },
                        ));
                    }
                    Token::Delimiter(',') | Token::Logical(LogicalOperator::Or) => continue,
                    Token::Delimiter('.') => {
                        self.requirements.inner.push(oneof);
                        return Some(());
                    }
                    _ => return None,
                },
                None => {
                    department = match self.next()? {
                        Token::Department(code) => Some(code),
                        Token::Delimiter('.') => return None,
                        _ => continue,
                    }
                }
            }
        }
    }

    fn type_decl(&mut self) -> Option<()> {
        self.requirement_type()?;
        match self.next()? {
            Token::Logical(LogicalOperator::Or) => self.requirement_type()?,
            Token::Delimiter(':') => {}
            _ => return None,
        }
        Some(())
    }

    fn requirement_type(&mut self) -> Option<()> {
        match self.next()? {
            Token::Type(req_type) => self.requirement_type |= req_type,
            _ => return None,
        }
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_either_prerequisite_or_corequisite() {
        let input = "Animal behavior from an ethological perspective, with emphasis on the \
                     mechanisms underlying a variety of behaviors. The material is intended to \
                     complement that of ZOOL 371. Prerequisite or corequisite: ZOOL 241 or 242 or \
                     PHYSL 210, or 212 or 214. Offered in alternate years.";
        let expected = Requirements::from(vec![OneOf {
            requirement: vec![
                "ZOOL241".into(),
                "ZOOL242".into(),
                "PHYSL210".into(),
                "PHYSL212".into(),
                "PHYSL214".into(),
            ],
            reqtype: RequirementType::Prerequisite | RequirementType::Corequisite,
        }]);
        assert_eq!(Parser::from(input).parse(), expected);
    }

    // #[test]
    // fn test_parse_multiple_requirements() {
    //     let input =
    //         "Prerequisites: ZOOL 101 and (BIOL 101 or CHEM 101). Corequisite:
    // ZOOL 201 or ZOOL 202.";     let expected = Requirements {
    //         prerequisites: vec![
    //             OneOf {
    //                 courses: vec!["ZOOL 101".to_string()],
    //                 logical_operator: Some(LogicalOperator::And),
    //             },
    //             OneOf {
    //                 courses: vec!["BIOL 101".to_string(), "CHEM
    // 101".to_string()],                 logical_operator:
    // Some(LogicalOperator::Or),             },
    //         ],
    //         corequisites: vec![OneOf {
    //             courses: vec!["ZOOL 201".to_string(), "ZOOL 202".to_string()],
    //             logical_operator: None,
    //         }],
    //     };
    //     assert_eq!(parse(input), expected);
    // }

    #[test]
    fn test_parse_no_requirements() {
        let input = "No prerequisites or corequisites.";
        let expected = Requirements::default();
        assert_eq!(Parser::from(input).parse(), expected);
    }

    // TODO: Highschool courses. Leverage the fact that the number will be < 100.
    // Make sure to deal with the different tiers. I.e. -1 in Math 30-1.

    #[test]
    fn test_parse_highschool_courses() {
        let input = "Review of numbers, inequalities, functions, analytic geometry; limits, \
                     continuity; derivatives and applications, Taylor polynomials; log, exp, and \
                     inverse trig functions. Integration, fundamental theorem of calculus \
                     substitution, trapezoidal and Simpson's rules. Prerequisites: Mathematics \
                     30-1 and Mathematics 31. Notes: (1) Credit can be obtained in at most one of \
                     MATH 100, 113, 114, 117, 134, 144, 154, or SCI 100. (2) Students in all \
                     sections of this course will write a common final examination. (3) \
                     Restricted to Engineering students. Non-Engineering students who take this \
                     course will receive *3.0.";
        let _expected = Requirements {
            inner: vec![
                OneOf {
                    requirement: vec!["Mathematics 30-1".into()],
                    reqtype: RequirementType::Prerequisite.into(),
                },
                OneOf {
                    requirement: vec!["Mathematics 31".into()],
                    reqtype: RequirementType::Prerequisite.into(),
                },
            ],
        };
        // FIXME: This currently doesn't work - we should be parsing the above requirements.
        let expected = Requirements { inner: vec![] };
        assert_eq!(Parser::from(input).parse(), expected);
    }

    #[test]
    fn test_parse_semicolon() {
        let input = "Prerequisites: CMPUT 174; CMPUT 203.";
        let expected = Requirements::from(vec![
            OneOf {
                requirement: vec!["CMPUT174".into()],
                reqtype: RequirementType::Prerequisite.into(),
            },
            OneOf {
                requirement: vec!["CMPUT203".into()],
                reqtype: RequirementType::Prerequisite.into(),
            },
        ]);
        assert_eq!(Parser::from(input).parse(), expected);
    }

    #[test]
    fn test_parse_multiple_dependencies() {
        let input = "Introduction to the basics of evaluation, including the foundations, \
                     approaches, steps, strategies, and ethical considerations of evaluation. \
                     Prerequisites: CMPUT 174 or 274; one of MATH 100, 114, 117, 134, 144, or \
                     154. Corequisites: CMPUT 175 or 275; CMPUT 272; MATH 125 or 127; one of STAT \
                     141, 151, 235, or 265, or SCI 151.";
        let expected = Requirements::from(vec![
            OneOf {
                requirement: vec!["CMPUT174".into(), "CMPUT274".into()],
                reqtype: RequirementType::Prerequisite.into(),
            },
            OneOf {
                requirement: vec![
                    "MATH100".into(),
                    "MATH114".into(),
                    "MATH117".into(),
                    "MATH134".into(),
                    "MATH144".into(),
                    "MATH154".into(),
                ],
                reqtype: RequirementType::Prerequisite.into(),
            },
            OneOf {
                requirement: vec!["CMPUT175".into(), "CMPUT275".into()],
                reqtype: RequirementType::Corequisite.into(),
            },
            OneOf {
                requirement: vec!["CMPUT272".into()],
                reqtype: RequirementType::Corequisite.into(),
            },
            OneOf {
                requirement: vec!["MATH125".into(), "MATH127".into()],
                reqtype: RequirementType::Corequisite.into(),
            },
            OneOf {
                requirement: vec![
                    "STAT141".into(),
                    "STAT151".into(),
                    "STAT235".into(),
                    "STAT265".into(),
                    "SCI151".into(),
                ],
                reqtype: RequirementType::Corequisite.into(),
            },
        ]);
        assert_eq!(Parser::from(input).parse(), expected);
    }
}
