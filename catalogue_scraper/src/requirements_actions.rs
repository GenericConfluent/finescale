use super::requirements::{Context, TokenKind};
/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as RustemoToken;
use std::fmt::Write as _;
pub type Ctx<'i> = Context<'i, str>;
pub type Token<'i> = RustemoToken<'i, str, TokenKind>;
#[derive(Clone, PartialEq, Eq)]
pub enum Expr {
    All(Vec<Expr>),
    Any(Vec<Expr>),
    Course(Course),
    Empty,
}
impl Expr {
    pub fn to_json(&self, mut obj: write_json::Object<'_>) {
        match self {
            Expr::All(args) => {
                let mut arr = obj.array("all");
                args.into_iter().for_each(|arg| arg.to_json(arr.object()));
            }
            Expr::Any(args) => {
                let mut arr = obj.array("any");
                args.into_iter().for_each(|arg| arg.to_json(arr.object()));
            }
            Expr::Course(Course { topic, number }) => {
                obj.object("course")
                    .string("topic", topic)
                    .string("number", &number.to_string());
            }
            Expr::Empty => {}
        };
    }
}
impl Expr {
    pub fn all(mut list: Vec<Expr>) -> Expr {
        list.retain(|expr| expr != &Expr::Empty);
        if list.is_empty() {
            Expr::Empty
        } else if list.len() == 1 {
            list.into_iter().nth(0).unwrap()
        } else {
            Expr::All(list)
        }
    }
    pub fn any(mut list: Vec<Expr>) -> Expr {
        list.retain(|expr| expr != &Expr::Empty);
        if list.is_empty() {
            Expr::Empty
        } else if list.len() == 1 {
            list.into_iter().nth(0).unwrap()
        } else {
            Expr::Any(list)
        }
    }
}
impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Any(args) => {
                write!(f, "(any")?;
                for arg in args {
                    write!(f, " {arg:?}")?;
                }
                f.write_char(')')?;
                Ok(())
            }
            Expr::All(args) => {
                write!(f, "(all")?;
                for arg in args {
                    write!(f, " {arg:?}")?;
                }
                f.write_char(')')?;
                Ok(())
            }
            Expr::Course(arg0) => write!(f, "({} {})", arg0.topic, arg0.number),
            Expr::Empty => write!(f, "()"),
        }
    }
}
impl From<Option<Expr>> for Expr {
    fn from(value: Option<Expr>) -> Expr {
        match value {
            Some(expr) => expr,
            None => Expr::Empty,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Course {
    pub topic: String,
    pub number: u64,
}
pub type AllCourses = Expr;
pub type AllCoursesSharedTopic = Expr;
pub type Alternative = ();
pub type AMinimumGradeIn = ();
pub type And = ();
pub type AndEt = ();
pub type AndOr = ();
pub type Any1 = Expr;
pub type Any2 = Expr;
pub type Any3 = Expr;
pub type Any3Like = Expr;
pub type AnyCourses = Expr;
pub type AnyCoursesSharedTopic = Expr;
pub type AnyCoursesSharedTopicNoCommas = Expr;
pub type Both = ();
pub type BothOpt = ();
pub type CommaOptAnd = ();
pub type CommaOptOr = ();
pub type CommaOrSemicolon = ();
pub type CommaOrSemicolonOpt = ();
pub type CommaSeparatedCourses2 = Vec<Expr>;
pub type CommaSeparatedCourses3 = Vec<Expr>;
pub type CommaSeparatedCoursesElement2 = Expr;
pub type CommaSeparatedCoursesElement20 = Vec<Expr>;
pub type CommaSeparatedCoursesElement21 = Vec<Expr>;
pub type CommaSeparatedCoursesElement3 = Expr;
pub type CommaSeparatedCoursesElement30 = Vec<Expr>;
pub type CommaSeparatedCoursesElement31 = Vec<Expr>;
pub type CoursesSharedTopicHead = (Topic, Vec<Number>);
pub type CoursesSharedTopicHeadElement = u64;
pub type CoursesSharedTopicHeadElement0 = Vec<u64>;
pub type CoursesSharedTopicHeadElement1 = Vec<u64>;
pub type CoursesSharedTopicSlashList = Expr;
pub type CoursesSharedTopicSlashListElement = u64;
pub type CoursesSharedTopicSlashListElement1 = Vec<u64>;
pub type Either = ();
pub type EitherCourse = Expr;
pub type EitherOpt = ();
pub type EnOrAlternativeOption = ();
pub type EnUnlessAlternativeOption = ();
pub type Et = ();
pub type FrOrAlternativeOption = ();
pub type None = ();
pub type Number = u64;
pub type OneOf = ();
pub type OneOfOpt = ();
pub type Or = ();
pub type OrAlternative = ();
pub type OrOu = ();
pub type Ou = ();
pub type Topic = String;
pub type TopLevel = Expr;
pub type TopLevelCommaList = TopLevelList;
pub type TopLevelCommaListElement = Any1;
pub type TopLevelCommaListElement0 = TopLevelCommaListElement1;
pub type TopLevelCommaListElement1 = Vec<TopLevelCommaListElement>;
pub type TopLevelCommaListLastElement = TopLevelListLastElement;
pub type TopLevelCommaListLastElementOpt = TopLevelCommaListLastElement;
pub type TopLevelList = Expr;
pub type TopLevelListLastElement = Expr;
pub type TopLevelSemicolonList = TopLevelCommaList;
pub type TopLevelSemicolonListElement = Any1;
pub type TopLevelSemicolonListElement0 = TopLevelSemicolonListElement1;
pub type TopLevelSemicolonListElement1 = Vec<TopLevelSemicolonListElement>;
pub type TopLevelSemicolonListLastElement = TopLevelListLastElement;
pub type TopLevelSemicolonListLastElementOpt = TopLevelSemicolonListLastElement;
pub type Unless = ();
pub type UnlessAlternative = ();
/// Functions are sorted alphabetically
/// To re-sort it in VSCode:
/// - Select "pub-fn" (written with a dash here so it's not selected)
/// - Hit ctrl-alt-l to select all instances
/// - Hit ctrl-shift-p then "Expand selection"
/// - Hit ctrl-shift-p then "rust-analyzer: Join lines" (twice if necessary)
/// - Select all the lines with functions
/// - Hit ctrl-shift-p then "Sort lines ascending"
const _: () = ();
pub fn all_courses_c1(
    _: &Ctx,
    (): (),
    mut list: Vec<Expr>,
    (): (),
    expr: Expr,
) -> AllCourses {
    list.push(expr);
    Expr::all(list)
}
pub fn all_courses_shared_topic_c1(
    _: &Ctx,
    (): (),
    (topic, mut numbers): (String, Vec<u64>),
    (): (),
    number: u64,
) -> Expr {
    numbers.push(number);
    Expr::all(
        numbers
            .into_iter()
            .map(|number| {
                Expr::Course(Course {
                    topic: topic.clone(),
                    number,
                })
            })
            .collect(),
    )
}
pub fn alternative(_: &Ctx, _: Token) {}
pub fn aminimum_grade_in(_: &Ctx, _: Token) {}
pub fn and_et_and_or(_: &Ctx, (): ()) {}
pub fn and_et_and(_: &Ctx, (): ()) {}
pub fn and_et_et(_: &Ctx, (): ()) {}
pub fn and_or(_: &Ctx, _: Token) {}
pub fn and(_: &Ctx, _: Token) {}
pub fn any_courses_c1(_: &Ctx, (): (), mut list: Vec<Expr>, (): (), expr: Expr) -> Expr {
    list.push(expr);
    Expr::any(list)
}
pub fn any_courses_shared_topic_c1(
    ctx: &Ctx,
    (): (),
    (topic, mut numbers): (String, Vec<u64>),
    (): (),
    number: u64,
) -> Expr {
    numbers.push(number);
    any_courses_shared_topic_c2(ctx, (), (topic, numbers))
}
pub fn any_courses_shared_topic_c2(
    _: &Ctx,
    (): (),
    (topic, nums): (String, Vec<u64>),
) -> Expr {
    Expr::any(
        nums
            .into_iter()
            .map(|number| {
                Expr::Course(Course {
                    topic: topic.clone(),
                    number,
                })
            })
            .collect(),
    )
}
pub fn any_courses_shared_topic_c3(
    ctx: &Ctx,
    (): (),
    (topic, numbers): (Topic, Vec<Number>),
    (): (),
    (): (),
) -> Expr {
    any_courses_shared_topic_c2(ctx, (), (topic, numbers))
}
pub fn any_courses_shared_topic_no_commas_c1(
    ctx: &Ctx,
    (): (),
    (topic, numbers): (Topic, Vec<Number>),
) -> Expr {
    any_courses_shared_topic_c2(ctx, (), (topic, numbers))
}
pub fn any1_all_courses_shared_topic(_: &Ctx, expr: Expr) -> Expr {
    expr
}
pub fn any1_all_courses(_: &Ctx, expr: AllCourses) -> Expr {
    expr
}
pub fn any1_any2(_: &Ctx, expr: Expr) -> Expr {
    expr
}
pub fn any1_c3(_: &Ctx, expr: Expr, (): ()) -> Expr {
    expr
}
pub fn any1_c4(_: &Ctx, expr: Expr, (): ()) -> Expr {
    expr
}
pub fn any2_any_courses_shared_topic(_: &Ctx, expr: Expr) -> Expr {
    expr
}
pub fn any2_any_courses(_: &Ctx, expr: Expr) -> Expr {
    expr
}
pub fn any2_any3(_: &Ctx, expr: Expr) -> Expr {
    expr
}
pub fn any2_c3(_: &Ctx, expr: Expr, (): ()) -> Expr {
    expr
}
pub fn any2_c4(_: &Ctx, expr: Expr, (): ()) -> Expr {
    expr
}
pub fn any3_alternative(_: &Ctx, (): ()) -> Expr {
    Expr::Empty
}
pub fn any3_any_courses_shared_topic_no_commas(
    _: &Ctx,
    expr: AnyCoursesSharedTopicNoCommas,
) -> Expr {
    expr
}
pub fn any3_c6(_: &Ctx, (): (), expr: Expr) -> Expr {
    expr
}
pub fn any3_course(_: &Ctx, course: Course) -> Expr {
    Expr::Course(course)
}
pub fn any3_courses_shared_topic_slash_list(_: &Ctx, expr: Expr) -> Expr {
    expr
}
pub fn any3_either_course(_: &Ctx, expr: Expr) -> Expr {
    expr
}
pub fn any3like_any3(_: &Ctx, expr: Expr) -> Expr {
    expr
}
pub fn any3like_en_or_alternative_option(_: &Ctx, (): ()) -> Expr {
    Expr::Empty
}
pub fn both_opt_both(_: &Ctx, (): Both) {}
pub fn both_opt_empty(_: &Ctx) {}
pub fn both(_: &Ctx, _: Token) {}
pub fn comma_opt_and_c1(_: &Ctx, (): (), (): ()) {}
pub fn comma_opt_or_c1(_: &Ctx, (): (), (): ()) {}
pub fn comma_or_semicolon_comma(_: &Ctx) {}
pub fn comma_or_semicolon_opt_comma_or_semicolon(_: &Ctx, (): ()) {}
pub fn comma_or_semicolon_opt_empty(_: &Ctx) {}
pub fn comma_or_semicolon_semicolon(_: &Ctx) {}
pub fn comma_separated_courses_element2_c1(_: &Ctx, (): (), expr: Expr) -> Expr {
    expr
}
pub fn comma_separated_courses_element20_comma_separated_courses_element21(
    _: &Ctx,
    list: Vec<Expr>,
) -> Vec<Expr> {
    list
}
pub fn comma_separated_courses_element20_empty(_: &Ctx) -> Vec<Expr> {
    vec![]
}
pub fn comma_separated_courses_element21_c1(
    _: &Ctx,
    mut list: Vec<Expr>,
    expr: Expr,
) -> Vec<Expr> {
    list.push(expr);
    list
}
pub fn comma_separated_courses_element21_comma_separated_courses_element2(
    _: &Ctx,
    expr: Expr,
) -> Vec<Expr> {
    vec![expr]
}
pub fn comma_separated_courses_element3_c1(_: &Ctx, (): (), expr: Expr) -> Expr {
    expr
}
pub fn comma_separated_courses_element30_comma_separated_courses_element31(
    _: &Ctx,
    list: Vec<Expr>,
) -> Vec<Expr> {
    list
}
pub fn comma_separated_courses_element30_empty(_: &Ctx) -> Vec<Expr> {
    vec![]
}
pub fn comma_separated_courses_element31_c1(
    _: &Ctx,
    mut list: Vec<Expr>,
    expr: Expr,
) -> Vec<Expr> {
    list.push(expr);
    list
}
pub fn comma_separated_courses_element31_comma_separated_courses_element3(
    _: &Ctx,
    list: Expr,
) -> Vec<Expr> {
    vec![list]
}
pub fn comma_separated_courses2_c1(
    _: &Ctx,
    expr: Expr,
    mut list: Vec<Expr>,
) -> Vec<Expr> {
    list.insert(0, expr);
    list
}
pub fn comma_separated_courses3_c1(
    _: &Ctx,
    expr: Expr,
    mut list: Vec<Expr>,
) -> Vec<Expr> {
    list.insert(0, expr);
    list
}
pub fn course_c1(_: &Ctx, topic: String, number: u64) -> Course {
    Course { topic, number }
}
pub fn courses_shared_topic_head_c1(
    _: &Ctx,
    topic: String,
    num: u64,
    mut nums: Vec<u64>,
) -> (String, Vec<u64>) {
    nums.insert(0, num);
    (topic, nums)
}
pub fn courses_shared_topic_head_element_c1(_: &Ctx, (): (), num: u64) -> u64 {
    num
}
pub fn courses_shared_topic_head_element0_courses_shared_topic_head_element1(
    _: &Ctx,
    list: Vec<u64>,
) -> Vec<u64> {
    list
}
pub fn courses_shared_topic_head_element0_empty(_: &Ctx) -> Vec<u64> {
    vec![]
}
pub fn courses_shared_topic_head_element1_c1(
    _: &Ctx,
    mut list: Vec<u64>,
    num: u64,
) -> Vec<u64> {
    list.push(num);
    list
}
pub fn courses_shared_topic_head_element1_courses_shared_topic_head_element(
    _: &Ctx,
    num: u64,
) -> Vec<u64> {
    vec![num]
}
pub fn courses_shared_topic_slash_list_c1(
    _: &Ctx,
    topic: String,
    num: u64,
    mut nums: Vec<u64>,
) -> Expr {
    nums.insert(0, num);
    Expr::any(
        nums
            .into_iter()
            .map(|number| {
                Expr::Course(Course {
                    topic: topic.clone(),
                    number,
                })
            })
            .collect(),
    )
}
pub fn courses_shared_topic_slash_list_element_c1(_: &Ctx, num: Number) -> u64 {
    num
}
pub fn courses_shared_topic_slash_list_element1_c1(
    _: &Ctx,
    mut list: Vec<u64>,
    num: u64,
) -> Vec<u64> {
    list.push(num);
    list
}
pub fn courses_shared_topic_slash_list_element1_courses_shared_topic_slash_list_element(
    _: &Ctx,
    num: u64,
) -> Vec<u64> {
    vec![num]
}
pub fn either_course_c1(_: &Ctx, (): (), lhs: Any3, (): (), rhs: Any3) -> Expr {
    Expr::any(vec![lhs, rhs])
}
pub fn either_opt_either(_: &Ctx, (): ()) {}
pub fn either_opt_empty(_: &Ctx) {}
pub fn either(_: &Ctx, _: Token) {}
pub fn en_or_alternative_option(_: &Ctx, _: Token) {}
pub fn en_unless_alternative_option(_: &Ctx, _: Token) {}
pub fn et(_: &Ctx, _: Token) {}
pub fn fr_or_alternative_option(_: &Ctx, _: Token) {}
pub fn none(_: &Ctx, _: Token) {}
pub fn number(_: &Ctx, token: Token) -> u64 {
    token.value.parse().unwrap()
}
pub fn one_of_opt_empty(_: &Ctx) {}
pub fn one_of_opt_one_of(_: &Ctx, (): ()) {}
pub fn one_of(_: &Ctx, _: Token) {}
pub fn or_alternative_c1(_: &Ctx, (): (), (): ()) {}
pub fn or_alternative_c2(_: &Ctx, (): (), (): ()) {}
pub fn or_ou_or(_: &Ctx, (): ()) {}
pub fn or_ou_ou(_: &Ctx, (): ()) {}
pub fn or(_: &Ctx, _: Token) {}
pub fn ou(_: &Ctx, _: Token) {}
pub fn top_level_any1(_: &Ctx, expr: Expr) -> Expr {
    expr
}
pub fn top_level_comma_list_c1(
    _: &Ctx,
    before: Expr,
    mut list: Vec<Expr>,
    after: Expr,
) -> Expr {
    list.insert(0, before);
    list.push(after);
    Expr::all(list)
}
pub fn top_level_comma_list_element_any1(_: &Ctx, expr: Expr) -> Expr {
    expr
}
pub fn top_level_comma_list_element0_empty(_: &Ctx) -> Vec<Expr> {
    vec![]
}
pub fn top_level_comma_list_element0_top_level_comma_list_element1(
    _: &Ctx,
    top_level_comma_list_element1: TopLevelCommaListElement1,
) -> TopLevelCommaListElement0 {
    top_level_comma_list_element1
}
pub fn top_level_comma_list_element1_c1(
    _: &Ctx,
    mut list: Vec<Expr>,
    expr: Expr,
) -> Vec<Expr> {
    list.push(expr);
    list
}
pub fn top_level_comma_list_element1_top_level_comma_list_element(
    _: &Ctx,
    top_level_comma_list_element: TopLevelCommaListElement,
) -> TopLevelCommaListElement1 {
    vec![top_level_comma_list_element]
}
pub fn top_level_comma_list_last_element_c1(_: &Ctx, (): (), expr: Expr) -> Expr {
    expr
}
pub fn top_level_comma_list_last_element_opt_empty(_: &Ctx) -> Expr {
    Expr::Empty
}
pub fn top_level_comma_list_last_element_opt_top_level_comma_list_last_element(
    _: &Ctx,
    top_level_comma_list_last_element: TopLevelCommaListLastElement,
) -> TopLevelCommaListLastElementOpt {
    top_level_comma_list_last_element
}
pub fn top_level_none(_: &Ctx, (): ()) -> Expr {
    Expr::Empty
}
pub fn top_level_semicolon_list_c1(
    _: &Ctx,
    before: Expr,
    mut list: Vec<Expr>,
    after: Expr,
) -> Expr {
    list.insert(0, before);
    list.push(after);
    Expr::all(list)
}
pub fn top_level_semicolon_list_element_any1(_: &Ctx, expr: Expr) -> Expr {
    expr
}
pub fn top_level_semicolon_list_element0_empty(_: &Ctx) -> Vec<Expr> {
    vec![]
}
pub fn top_level_semicolon_list_element0_top_level_semicolon_list_element1(
    _: &Ctx,
    top_level_semicolon_list_element1: TopLevelSemicolonListElement1,
) -> TopLevelSemicolonListElement0 {
    top_level_semicolon_list_element1
}
pub fn top_level_semicolon_list_element1_c1(
    _: &Ctx,
    mut list: Vec<Expr>,
    expr: Expr,
) -> Vec<Expr> {
    list.push(expr);
    list
}
pub fn top_level_semicolon_list_element1_top_level_semicolon_list_element(
    _: &Ctx,
    top_level_semicolon_list_element: TopLevelSemicolonListElement,
) -> TopLevelSemicolonListElement1 {
    vec![top_level_semicolon_list_element]
}
pub fn top_level_semicolon_list_last_element_c1(_: &Ctx, (): (), expr: Expr) -> Expr {
    expr
}
pub fn top_level_semicolon_list_last_element_opt_empty(_: &Ctx) -> Expr {
    Expr::Empty
}
pub fn top_level_semicolon_list_last_element_opt_top_level_semicolon_list_last_element(
    _: &Ctx,
    top_level_semicolon_list_last_element: TopLevelSemicolonListLastElement,
) -> TopLevelSemicolonListLastElementOpt {
    top_level_semicolon_list_last_element
}
pub fn top_level_top_level_comma_list(
    _: &Ctx,
    top_level_comma_list: TopLevelCommaList,
) -> TopLevel {
    top_level_comma_list
}
pub fn top_level_top_level_semicolon_list(
    _: &Ctx,
    top_level_semicolon_list: TopLevelSemicolonList,
) -> TopLevel {
    top_level_semicolon_list
}
pub fn topic(_: &Ctx, token: Token) -> String {
    token.value.into()
}
pub fn unless_alternative_c1(_: &Ctx, (): (), (): ()) {}
pub fn unless(_: &Ctx, _: Token) {}
