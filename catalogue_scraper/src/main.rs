mod requirement_extractor;
#[cfg(test)]
mod requirements_tests;
rustemo::rustemo_mod!(requirements, "/src");
mod requirements_actions;

use crate::requirement_extractor::extract_requirement_strings;
use anyhow::{Context, Result};
use log::{debug, info};
use rate_limit::UnsyncLimiter;
use requirement_extractor::RequirementKind;
use requirements_actions::Expr;
use rustemo::Parser;
use std::borrow::Cow;
use std::collections::HashMap;
use std::io::{stdout, ErrorKind, Write};
use std::path::PathBuf;
use std::time::Duration;
use url::Url;

struct Cache {
    base_url: Url,
    base_path: PathBuf,
    rate_limiter: UnsyncLimiter,
}

impl Cache {
    fn new(base_url: Url, base_path: PathBuf, rate_limiter: UnsyncLimiter) -> Result<Cache> {
        std::fs::create_dir_all(&base_path)?;
        Ok(Cache {
            base_url,
            base_path,
            rate_limiter,
        })
    }

    fn get(&mut self, path: &str) -> Result<String> {
        let full_url = self.base_url.join(path)?;
        let encoded_url = urlencoding::encode(full_url.as_str());
        let full_path = self.base_path.join(&*encoded_url);
        match std::fs::read_to_string(&full_path) {
            Ok(content) => {
                debug!("got {full_url} from cache");
                Ok(content)
            }
            Err(e) if e.kind() == ErrorKind::NotFound => {
                match self.rate_limiter.consume(1) {
                    Ok(_) => {}
                    Err(duration) => {
                        debug!("hit rate limit; waiting {duration:?}...");
                        std::thread::sleep(duration);
                    }
                }
                debug!("requesting {full_url}");
                let response = ureq::get(full_url.as_str()).call()?;
                let mut content = String::new();
                response.into_reader().read_to_string(&mut content)?;
                std::fs::write(&full_path, &content)?;
                info!("requested and cached {full_url}");
                Ok(content)
            }
            Err(e) => Err(e.into()),
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let rate_limit = UnsyncLimiter::new(1, 1, Duration::from_secs(15));
    let mut cacher = Cache::new(
        Url::parse("https://apps.ualberta.ca")?,
        PathBuf::from("cache"),
        rate_limit,
    )?;

    let mut writer = stdout().lock();
    let catalogue_html = cacher.get("/catalogue/course")?;
    let catalogue_vdom = tl::parse(&catalogue_html, tl::ParserOptions::default())?;
    for dept_link in get_links(&catalogue_vdom, |s| s.starts_with("/catalogue/course/")) {
        let wanted_prefix = format!("{dept_link}/");
        let dept_html = cacher.get(&dept_link)?;
        let dept_vdom = tl::parse(&dept_html, tl::ParserOptions::default())?;
        for course_link in get_links(&dept_vdom, |s| s.starts_with(&wanted_prefix)) {
            let course_html = cacher.get(&course_link)?;
            let course_vdom = tl::parse(&course_html, tl::ParserOptions::default())?;
            let course_meta = get_course_meta(&course_vdom)
                .with_context(|| format!("course meta for {course_link} failed"))?;
            let course_desc = get_course_desc(&course_vdom)
                .with_context(|| format!("course desc for {course_link} failed"))?;
            let desc_or_empty = course_desc.as_deref().unwrap_or_default();
            let (prereqs, coreqs) = extract_parsed_requirements(desc_or_empty);
            let course_data = CourseData {
                meta: course_meta,
                desc: course_desc,
                prereqs,
                coreqs,
            };
            writer.write_all(course_data.to_json_string().as_bytes())?;
            writer.write_all(b"\n")?;
        }
    }

    Ok(())
}

fn extract_parsed_requirements(course_description: &str) -> (Expr, Expr) {
    let parsed_reqs = extract_requirement_strings(course_description)
        .into_iter()
        .filter_map(|(kind, req)| {
            let parser = requirements::RequirementsParser::new();
            match parser.parse(req) {
                Ok(forest) => match forest.get_first_tree() {
                    Some(tree) => {
                        let mut builder = requirements::DefaultBuilder::new();
                        let expr = tree.build(&mut builder);
                        Some((kind, expr))
                    }
                    None => None,
                },
                Err(_) => None,
            }
        });

    let mut prereqs = Expr::Empty;
    let mut coreqs = Expr::Empty;
    for (kind, expr) in parsed_reqs {
        match kind {
            RequirementKind::Prerequisite => prereqs = Expr::all(vec![prereqs, expr]),
            RequirementKind::Corequisite => coreqs = Expr::all(vec![coreqs, expr]),
        }
    }
    (prereqs, coreqs)
}

#[derive(Debug, Clone)]
struct CourseData<'a> {
    meta: CourseMeta<'a>,
    desc: Option<Cow<'a, str>>,
    prereqs: Expr,
    coreqs: Expr,
}

impl CourseData<'_> {
    fn to_json_string(&self) -> String {
        let mut buf = String::new();
        let mut data = write_json::object(&mut buf);
        let mut meta = data.object("meta");
        meta.string("faculty", &self.meta.faculty);
        meta.string("subject", &self.meta.subject);
        meta.string("catalog", &self.meta.catalog);
        meta.string("course", &self.meta.course);
        meta.string("coursetitle", &self.meta.coursetitle);
        meta.string("credits", &self.meta.credits);
        meta.string("career", &self.meta.career);
        meta.string("term", &self.meta.term);
        meta.string("sections", &self.meta.sections);
        meta.string("sections_online", &self.meta.sections_online);
        drop(meta);
        match &self.desc {
            Some(it) => data.string("desc", it),
            None => data.null("desc"),
        };
        self.prereqs.to_json(data.object("prereqs"));
        self.coreqs.to_json(data.object("coreqs"));
        drop(data);
        buf
    }
}

#[derive(Debug, Clone)]
struct CourseMeta<'a> {
    faculty: Cow<'a, str>,
    subject: Cow<'a, str>,
    catalog: Cow<'a, str>,
    course: Cow<'a, str>,
    coursetitle: Cow<'a, str>,
    credits: Cow<'a, str>,
    career: Cow<'a, str>,
    term: Cow<'a, str>,
    sections: Cow<'a, str>,
    sections_online: Cow<'a, str>,
}

fn get_links<'a>(vdom: &'a tl::VDom<'a>, predicate: impl Fn(&str) -> bool) -> Vec<Cow<'a, str>> {
    let mut links: Vec<_> = vdom
        .nodes()
        .iter()
        .filter_map(|node| match node {
            tl::Node::Tag(tag) if tag.name() == "a" => tag
                .attributes()
                .get("href")
                .flatten()
                .map(|attr_val| attr_val.as_utf8_str())
                .filter(|link| predicate(link)),
            _ => None,
        })
        .collect();
    links.sort();
    links.dedup();
    links
}

fn get_course_meta<'a>(vdom: &'a tl::VDom<'a>) -> Option<CourseMeta<'a>> {
    let metas: HashMap<Cow<'_, str>, Cow<'_, str>> = vdom
        .nodes()
        .iter()
        .filter_map(|node| match node {
            tl::Node::Tag(tag) if tag.name() == "meta" => {
                let name = tag.attributes().get("name").flatten()?;
                let content = tag.attributes().get("content").flatten()?;
                Some((name.as_utf8_str(), content.as_utf8_str()))
            }
            _ => None,
        })
        .collect();
    let get = |name: &str| metas.get(name).cloned().map(cow_decode_html_entities);
    Some(CourseMeta {
        faculty: get("ua__cat_faculty")?,
        subject: get("ua__cat_subject")?,
        catalog: get("ua__cat_catalog")?,
        course: get("ua__cat_course")?,
        coursetitle: get("ua__cat_coursetitle")?,
        credits: get("ua__cat_credits")?,
        career: get("ua__cat_career")?,
        term: get("ua__cat_term")?,
        sections: get("ua__cat_sections")?,
        sections_online: get("ua__cat_sections_online")?,
    })
}

fn get_course_desc<'a>(vdom: &'a tl::VDom<'a>) -> Option<Option<Cow<'a, str>>> {
    // I would use `* > nav` or something here but query_selector is only partially
    // implemented.
    vdom.get_elements_by_class_name("container")
        .map(|handle| handle.get(vdom.parser()).unwrap())
        .filter(|node| {
            node.children().is_some_and(|children| {
                children
                    .all(vdom.parser())
                    .iter()
                    .any(|child| child.as_tag().is_some_and(|tag| tag.name() == "nav"))
            })
        })
        .find_map(|container| {
            container
                .children()
                .unwrap()
                .all(vdom.parser())
                .iter()
                .filter_map(tl::Node::as_tag)
                .filter(|tag| {
                    tag.name() == "p"
                        || (tag.name() == "div"
                            && tag.attributes().is_class_member("alert")
                            && !tag.inner_text(vdom.parser()).contains(
                                "There are currently no scheduled offerings of this course.",
                            ))
                })
                .nth(1)
                .map(|node| match node.name().as_bytes() {
                    b"p" => Some(cow_decode_html_entities(node.inner_text(vdom.parser()))),
                    b"div" => None,
                    _ => unreachable!(),
                })
        })
}

fn cow_decode_html_entities(desc: Cow<'_, str>) -> Cow<'_, str> {
    match desc {
        Cow::Borrowed(s) => html_escape::decode_html_entities(s.trim()),
        Cow::Owned(s) => Cow::Owned(html_escape::decode_html_entities(&s.trim()).into_owned()),
    }
}
