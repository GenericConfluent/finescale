use anyhow::anyhow;
use core::str::FromStr;
use petgraph::dot::Dot;
use petgraph::graph::{DiGraph, NodeIndex};
use serde::Deserialize;
use std::fmt;

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
            .filter(|string| !string.trim_start().is_empty());

        let subject_id = pieces
            .next()
            .ok_or(anyhow!("Could not find a subject id"))?
            .to_uppercase()
            .to_string();
        let class_id = pieces
            .next()
            .ok_or(anyhow!("Could not find the class id"))?
            .parse()?;

        if !(100..1000).contains(&class_id) {
            return Err(anyhow!(
                "Class id must be a number, at least 100 and less than 1000"
            ));
        }

        Ok(CourseId {
            subject_id,
            class_id,
        })
    }
}

impl std::fmt::Display for CourseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.subject_id, self.class_id)
    }
}

#[derive(Deserialize, Clone, Debug)]
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

#[derive(Debug)]
pub enum NodeType {
    Course(Course),
    Or,
}

#[derive(Debug)]
pub struct Node {
    pub ntype: NodeType,
    pub val: u16,
}

impl Node {
    fn has_id(&self, id: &CourseId) -> bool {
        match &self.ntype {
            NodeType::Course(course) => course.id == *id,
            _ => false,
        }
    }
}

impl From<Course> for Node {
    fn from(value: Course) -> Self {
        Self {
            ntype: NodeType::Course(value),
            val: 0,
        }
    }
}

impl From<NodeType> for Node {
    fn from(ntype: NodeType) -> Self {
        Self { ntype, val: 0 }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use NodeType::*;
        match &self.ntype {
            Or => write!(f, "OR"),
            Course(c) => write!(f, "{}", c.id),
        }
    }
}

/// Abstraction over some way to retrieve course info for simplicity.
/// There must only be one entry for each course.
#[derive(Debug)]
pub struct CourseGraph {
    pub courses: DiGraph<Node, Relation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relation {
    Prereq,
    Coreq,
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
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

impl CourseGraph {
    pub fn new(source: &str) -> anyhow::Result<Self> {
        // Get list of unique courses sorted
        let mut course_list: Vec<Course> = ron::from_str(source)?;
        course_list.sort_unstable_by(|a, b| a.id.cmp(&b.id));
        course_list.dedup_by(|a, b| a.id == b.id);

        // Build the course graph
        let mut courses = DiGraph::new();
        let mut edge_queue = Vec::<(NodeIndex, CourseId, Relation)>::new();

        fn descend_deptree(
            courses: &mut DiGraph<Node, Relation>,
            edge_queue: &mut Vec<(NodeIndex, CourseId, Relation)>,
            node: &NodeIndex,
            requirement: &Requirement,
        ) {
            match requirement {
                Requirement::And(req_list) | Requirement::Or(req_list) => {
                    let node = if let Requirement::Or(_) = requirement {
                        let id = courses.add_node(NodeType::Or.into());
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
                    // Note that A corequisite B && B !corequisite A is true.
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

    pub fn index_of(&self, id: &CourseId) -> Option<NodeIndex> {
        let idx = self
            .courses
            .node_indices()
            .find(|node_idx| self.courses[*node_idx].has_id(id))?;
        Some(match &self.courses[idx].ntype {
            NodeType::Course(course) => idx,
            _ => unreachable!(),
        })
    }

    pub fn to_dot(&self) -> String {
        format!("{}", Dot::new(&self.courses))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::visit::EdgeRef;

    static CMPUT_SMALL: &str = r#"[
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
    #[track_caller]
    fn assert_in_db(db: &CourseGraph, id: &CourseId) -> NodeIndex {
        let Some(course_idx) = db.index_of(id) else {
            panic!("{} not in the Database", id);
        };

        course_idx
    }

    #[test]
    fn cmput_small() {
        let db = match CourseGraph::new(CMPUT_SMALL) {
            Ok(db) => db,
            Err(err) => panic!("Faild to build CourseDatabase {:?}", err),
        };

        let cmput_101_id = CourseId {
            subject_id: "CMPUT".into(),
            class_id: 101,
        };
        let math_112_id = CourseId {
            subject_id: "MATH".into(),
            class_id: 112,
        };

        let cmput_101 = assert_in_db(&db, &cmput_101_id);
        let cmput_102 = assert_in_db(
            &db,
            &CourseId {
                subject_id: "CMPUT".into(),
                class_id: 102,
            },
        );
        let math_111 = assert_in_db(
            &db,
            &CourseId {
                subject_id: "MATH".into(),
                class_id: 111,
            },
        );
        let math_112 = assert_in_db(&db, &math_112_id);

        assert_eq!(
            db.courses.edges(cmput_101).count(),
            0,
            "CMPUT 101 was parsed as having dependencies when it has none"
        );
        let mut desired = vec![cmput_101_id, math_112_id];

        for edge in db.courses.edges(cmput_102) {
            assert_eq!(
                *edge.weight(),
                Relation::Prereq,
                "All dependencies of CMPUT 102 should be prereqs"
            );
            assert_eq!(
                edge.source(),
                cmput_102,
                "There is something wrong with petgraph or the test"
            );
            let Some((idx, cid)) = desired
                .iter()
                .enumerate()
                .find(|cid| db.courses[edge.target()].has_id(cid.1))
            else {
                panic!("There was an unexpected dependency on CMPUT 102");
            };
            desired.remove(idx);
        }

        assert!(desired.is_empty(), "CMPUT 101 had dependencies missing");
    }
}
