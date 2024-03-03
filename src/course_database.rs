use anyhow::anyhow;
use core::str::FromStr;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::Graph;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::io::BufReader;

#[derive(Deserialize, Clone, Debug)]
pub enum Requirement {
    And(Vec<Requirement>),
    Or(Vec<Requirement>),
    Prereq(CourseId),
    Coreq(CourseId),
}

impl Requirement {
    fn unwrap(&self) -> CourseId {
        use Requirement::*;
        match self {
            Prereq(id) | Coreq(id) => id.clone(),
            _ => panic!("Tried to unwrap requirement {:?}", self),
        }
    }
}

// Note: Do not change the field orders. When deriving ord comparision is done
// in top to bottom order. We want subject_id to be compared first so that all
// classes with the same subject_id will be grouped together in memory when a
// course list is sorted.
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(Deserialize, Clone)]
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

pub enum DatabaseNode {
    Course(Course),
    Or,
}

impl DatabaseNode {
    fn has_id(&self, id: &CourseId) -> bool {
        match self {
            DatabaseNode::Course(course) => course.id == *id,
            _ => false,
        }
    }
}

impl From<Course> for DatabaseNode {
    fn from(value: Course) -> Self {
        DatabaseNode::Course(value)
    }
}

/// Abstraction over some way to retrieve course info for simplicity.
/// There must only be one entry for each course.
pub struct CourseDatabase {
    courses: Graph<DatabaseNode, Relation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relation {
    Prereq,
    Coreq,
}

impl TryFrom<&Requirement> for Relation {
    type Error = ();

    fn try_from(value: &Requirement) -> Result<Self, Self::Error> {
        use Requirement::*;
        Ok(match value {
            Prereq(_) => Relation::Prereq,
            Coreq(_) => Relation::Coreq,
            _ => return Err(()),
        })
    }
}

impl CourseDatabase {
    pub fn new(source: &str) -> anyhow::Result<Self> {
        // Get list of unique courses sorted
        let mut course_list: Vec<Course> = ron::from_str(source)?;
        course_list.sort_unstable_by(|a, b| a.id.cmp(&b.id));
        course_list.dedup_by(|a, b| a.id == b.id);

        // Build the course graph
        let mut courses = Graph::new();
        let mut edge_queue = Vec::<(NodeIndex, CourseId, Relation)>::new();

        fn descend_deptree(
            courses: &mut Graph<DatabaseNode, Relation>,
            edge_queue: &mut Vec<(NodeIndex, CourseId, Relation)>,
            node: &NodeIndex,
            requirement: &Requirement,
        ) {
            match requirement {
                Requirement::And(req_list) | Requirement::Or(req_list) => {
                    let node = if let Requirement::Or(_) = requirement {
                        let id = courses.add_node(DatabaseNode::Or);
                        courses.add_edge(*node, id, Relation::Prereq);
                        id
                    } else {
                        *node
                    };
                    for req in req_list {
                        descend_deptree(courses, edge_queue, &node, req);
                    }
                }
                req => {
                    edge_queue.push((*node, req.unwrap(), req.try_into().unwrap()));
                }
            }
        }

        let mut remove_from_queue = Vec::new();

        for course in course_list {
            let course_id = course.id.clone();
            let reqs = course.requirements.clone();
            let node = courses.add_node(course.into());

            // Add edges to nodes from previous iterations.
            for (idx, (node_from, _, relation)) in edge_queue
                .iter()
                .enumerate()
                .filter(|edge| edge.1 .1 == course_id)
            {
                courses.add_edge(*node_from, node, *relation);
                remove_from_queue.push(idx);
            }

            for idx in &remove_from_queue {
                edge_queue.remove(*idx);
            }
            remove_from_queue.clear();

            if let Some(req) = reqs {
                descend_deptree(&mut courses, &mut edge_queue, &node, &req);
            }
        }

        // [A, B, C] where course B depends on A, and C. The for loop above ensures C
        // and any dependencies after B are added. Now we need to go through the
        // remaining edges in the queue.
        for (node_from, target_id, relation) in edge_queue {
            let node_to = courses
                .node_indices()
                .find(|node_idx| courses[*node_idx].has_id(&target_id))
                .expect("An invalid node edge was added");
            courses.add_edge(node_from, node_to, relation);
        }

        Ok(Self { courses })
    }

    pub fn get(&self, id: &CourseId) -> Option<Course> {
        let idx = self
            .courses
            .node_indices()
            .find(|node_idx| self.courses[*node_idx].has_id(id))?;
        Some(match &self.courses[idx] {
            DatabaseNode::Course(course) => course.clone(),
            _ => unreachable!(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static CMPUT_SMALL: &'static str = r#"[
(
    id: (subject_id: "CMPUT", class_id: 101),
    name: "",
    description: "",
    requirements: None,
),
(
    id: (subject_id: "CMPUT", class_id: 102),
    name: "",
    description: "",
    requirements: Some(And([
        Prereq((subject_id: "CMPUT", class_id: 101)),
        Prereq((subject_id: "MATH", class_id: 112)),
    ])),
),
(
    id: (subject_id: "MATH", class_id: 111),
    name: "",
    description: "",
    requirements: None,
),
(
    id: (subject_id: "MATH", class_id: 112),
    name: "",
    description: "",
    requirements: Some(Prereq((subject_id: "MATH", class_id: 111))),
),
]"#;

    #[test]
    fn cmput_small() {
        let cd = match CourseDatabase::new(CMPUT_SMALL) {
            Ok(cd) => cd,
            Err(err) => panic!("Faild to build CourseDatabase {:?}", err),
        };

        assert!(
            matches!(
                cd.get(&CourseId {
                    subject_id: "CMPUT".into(),
                    class_id: 101
                }),
                Some(_)
            ),
            "CMPUT 101 not in the Database"
        );
    }
}

// impl Default for CourseDatabase {
//     // Simple course catalog to use for testing.
//     fn default() -> Self {
//         use Requirement::*;
//         let cmput_101 = Course::new("CMPUT", 101, None);
//         let cmput_102 = Course::new("CMPUT", 102,
// Some(Prereq(cmput_101.id.clone())));
//
//         let math_111 = Course::new("MATH", 111, None);
//         let math_112 = Course::new("MATH", 112,
// Some(Prereq(math_111.id.clone())));
//
//         let cmput_174 = Course::new(
//             "CMPUT",
//             174,
//             Some(And(vec![
//                 Prereq(math_112.id.clone()),
//                 Coreq(cmput_102.id.clone()),
//             ])),
//         );
//         let cmput_175 = Course::new("CMPUT", 175,
// Some(Prereq(cmput_174.id.clone())));
//
//         let cmput_274 = Course::new("CMPUT", 274, None);
//         let cmput_275 = Course::new("CMPUT", 275,
// Some(Prereq(cmput_274.id.clone())));
//
//         let cmput_202 = Course::new("CMPUT", 202, None);
//
//         let cmput_322 = Course::new(
//             "CMPUT",
//             322,
//             Some(And(vec![
//                 Or(vec![
//                     Requirement::Prereq(cmput_275.id.clone()),
//                     Requirement::Prereq(cmput_175.id.clone()),
//                 ]),
//                 Requirement::Prereq(cmput_202.id.clone()),
//             ])),
//         );
//         Self {
//             courses: vec![
//                 cmput_101, cmput_102, math_111, math_112, cmput_174,
// cmput_175, cmput_274,                 cmput_275, cmput_202, cmput_322,
//             ],
//         }
//     }
// }
