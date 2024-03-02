use anyhow::anyhow;
use core::str::FromStr;
use petgraph::graph::DiGraph;
use std::fmt;

pub enum Requirement {
    And(Vec<Requirement>),
    Or(Vec<Requirement>),
    Prereq(CourseId),
    Coreq(CourseId),
}

#[derive(Clone)]
pub struct CourseId {
    pub subject_id: String,
    pub class_id: u16,
}

impl FromStr for CourseId {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut pieces = value
            .split_whitespace()
            .filter(|string| string.trim_start().len() != 0);

        Ok(CourseId {
            subject_id: pieces
                .next()
                .ok_or(anyhow!("Could not find a subject id"))?
                .to_string(),
            class_id: pieces
                .next()
                .ok_or(anyhow!("Could not find the class id"))?
                .parse()?,
        })
    }
}

impl std::fmt::Display for CourseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.subject_id, self.class_id)
    }
}

pub struct Course {
    pub id: CourseId,
    pub name: String,
    pub description: String,
    pub requirements: Option<Requirement>,
}

impl Course {
    pub fn new(subject_id: &str, class_id: u16, requirements: Option<Requirement>) -> Self {
        let id = CourseId {
            subject_id: subject_id.into(),
            class_id,
        };
        Course {
            id,
            name: String::new(),
            description: String::new(),
            requirements,
        }
    }
}

/// Abstraction over some way to retrieve course info for simplicity.
pub struct CourseDatabase {
    courses: Vec<Course>,
}

impl Default for CourseDatabase {
    // Simple course catalog to use for testing.
    fn default() -> Self {
        use Requirement::*;
        let cmput_101 = Course::new("CMPUT", 101, None);
        let cmput_102 = Course::new("CMPUT", 102, Some(Prereq(cmput_101.id.clone())));

        let math_111 = Course::new("MATH", 111, None);
        let math_112 = Course::new("MATH", 112, Some(Prereq(math_111.id.clone())));

        let cmput_174 = Course::new(
            "CMPUT",
            174,
            Some(And(vec![
                Prereq(math_112.id.clone()),
                Coreq(cmput_102.id.clone()),
            ])),
        );
        let cmput_175 = Course::new("CMPUT", 175, Some(Prereq(cmput_174.id.clone())));

        let cmput_274 = Course::new("CMPUT", 274, None);
        let cmput_275 = Course::new("CMPUT", 275, Some(Prereq(cmput_274.id.clone())));

        let cmput_202 = Course::new("CMPUT", 202, None);

        let cmput_322 = Course::new(
            "CMPUT",
            322,
            Some(And(vec![
                Or(vec![
                    Requirement::Prereq(cmput_275.id.clone()),
                    Requirement::Prereq(cmput_175.id.clone()),
                ]),
                Requirement::Prereq(cmput_202.id.clone()),
            ])),
        );
        Self {
            courses: vec![
                cmput_101, cmput_102, math_111, math_112, cmput_174, cmput_175, cmput_274,
                cmput_275, cmput_202, cmput_322,
            ],
        }
    }
}
