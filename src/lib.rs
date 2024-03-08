#![allow(dead_code, unused_variables)]

use core::panic;
use std::collections::VecDeque;
use std::sync::Arc;

use iced::alignment::Horizontal;
use iced::widget::{button, column, container, horizontal_rule, row, text, text_input, Text};
use iced::{executor, font, Application, Color, Command, Element, Length, Theme};

use iced_aw::native::Split;
use iced_aw::{modal, split, Card};

mod course_database;
use course_database::{CourseGraph, CourseId, CourseSet, NodeType};
use icons::Icon;

use anyhow::anyhow;

mod graph_widget;
mod icons;

#[derive(Default)]
pub struct FinescaleApp {
    // This should be sorted.
    desired_courses: Vec<CourseId>,
    required_courses: Vec<CourseSet>,
    course_graph: Option<CourseGraph>,
    ui_states: UiStates,
}

#[derive(Default)]
struct UiStates {
    error_modal: VecDeque<String>,
    main_divider_pos: Option<u16>,
    course_input_val: String,
}

/// UI events for the app. Do not clone `Message::LoadedCourses(...)`, and don't
/// put an Arc with a strong refcount != 1 in the variant.
#[derive(Debug, Clone)]
pub enum Message {
    LoadedCourses(Arc<anyhow::Result<CourseGraph>>),
    MainDividerResize(u16),
    CourseInputEvent(String),
    CourseInputSubmit,
    CourseDelete(usize),
    IconsLoaded(Result<(), font::Error>),
    // TODO: Invoke GraphViz as a subprocess and use the `open` crate to show its output.
    ShowGraph,
    ExportToSchedubuddy,
    ClearError,
}

// NOTE: May make sense to compress all the files into an archive and
// download from Github on each startup.
async fn load_courses<P: AsRef<std::path::Path>>(path: P) -> Arc<anyhow::Result<CourseGraph>> {
    CourseGraph::new("[]").into()
}

impl FinescaleApp {
    fn update_required_courses(&mut self) -> anyhow::Result<()> {
        if self.desired_courses.is_empty() {
            self.course_graph = None;
            return Ok(());
        }

        let graph = self
            .course_graph
            .as_mut()
            .ok_or(anyhow!("Course database is not yet loaded."))?;
        let mut desired_courses = Vec::with_capacity(self.desired_courses.len());

        for course_id in &self.desired_courses {
            let Some(idx) = graph.index_of(course_id) else {
                self.ui_states.error_modal.push_back(format!(
                    "{} is not in course database. So its requirements will not be accounted for.",
                    course_id
                ));
                continue;
            };
            desired_courses.push(idx);
        }

        graph.count_dependents(&desired_courses)?;
        self.required_courses = graph.build_sets(&desired_courses, 5);
        Ok(())
    }

    fn open_in_schedubuddy(&self) -> anyhow::Result<()> {
        Err(anyhow!("Export to schedubudy still needs implementation"))
    }

    fn open_graphviz(&self) -> anyhow::Result<()> {
        Err(anyhow!("Open in graphviz still needs implementation"))
    }
}

impl Application for FinescaleApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            FinescaleApp::default(),
            Command::batch([
                Command::perform(load_courses("data/courses.ron"), Message::LoadedCourses),
                iced::font::load(icons::Icon::bytes()).map(Message::IconsLoaded),
            ]),
        )
    }

    fn title(&self) -> String {
        "Finescale".to_string()
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        match _message {
            // NOTE: The Arc is just being used to transport the data and allow Clone
            // to be derived on Message.
            Message::LoadedCourses(result_ptr) => match Arc::into_inner(result_ptr) {
                Some(Ok(db)) => self.course_graph = Some(db),
                Some(Err(issue)) => self.ui_states.error_modal.push_back(issue.to_string()),
                _ => panic!("Read the docs on Message"),
            },
            Message::CourseDelete(idx) => {
                _ = self.desired_courses.remove(idx);
                _ = self
                    .update_required_courses()
                    .inspect_err(|e| self.ui_states.error_modal.push_back(e.to_string()));
            }
            Message::ClearError => _ = self.ui_states.error_modal.pop_front(),
            // TODO: Limit the divider movement
            Message::MainDividerResize(amt) => self.ui_states.main_divider_pos = Some(amt),
            Message::CourseInputEvent(val) => self.ui_states.course_input_val = val,
            Message::CourseInputSubmit => {
                match self.ui_states.course_input_val.parse::<CourseId>() {
                    Ok(course) => {
                        let Err(idx) = self.desired_courses.binary_search(&course) else {
                            self.ui_states
                                .error_modal
                                .push_back(format!("{} has already been added.", course));
                            return Command::none();
                        };

                        self.desired_courses.insert(idx, course);
                        self.ui_states.course_input_val.clear();

                        _ = self
                            .update_required_courses()
                            .inspect_err(|e| self.ui_states.error_modal.push_back(e.to_string()));
                    }
                    Err(issue) => self.ui_states.error_modal.push_back(issue.to_string()),
                }
            }
            // TODO: This should be taking a semester of courses, but there is no functionality yet
            // to organize all the desired coureses and required courses into semesters.
            Message::ExportToSchedubuddy => {
                _ = self
                    .open_in_schedubuddy()
                    .inspect_err(|e| self.ui_states.error_modal.push_back(e.to_string()))
            }
            Message::ShowGraph => {
                _ = self
                    .open_graphviz()
                    .inspect_err(|e| self.ui_states.error_modal.push_back(e.to_string()))
            }
            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut left = column![
            text("Desired Classes")
                .width(Length::Fill)
                .size(40)
                .style(Color::from_rgb(0.5, 0.5, 0.5))
                .horizontal_alignment(Horizontal::Center),
            text_input("Start typing!", &self.ui_states.course_input_val)
                .padding(15)
                .on_input(Message::CourseInputEvent)
                .on_submit(Message::CourseInputSubmit),
        ]
        .spacing(10);

        let mut right = column![
            row![
                text("Required Classes")
                    .width(Length::Fill)
                    .size(40)
                    .style(Color::from_rgb(0.5, 0.5, 0.5))
                    .horizontal_alignment(iced::alignment::Horizontal::Left),
                button(Text::from(Icon::FullStackedBarChart))
                    .padding(10)
                    .on_press(Message::ExportToSchedubuddy),
                button(Text::from(Icon::AccountTree))
                    .padding(10)
                    .on_press(Message::ShowGraph)
            ],
            horizontal_rule(2)
        ];

        for (idx, course) in self.desired_courses.iter().enumerate() {
            left = left.push(
                row![
                    text(course).width(Length::Fill),
                    button(Text::from(Icon::DeleteForever))
                        .padding(10)
                        .on_press(Message::CourseDelete(idx))
                ]
                .spacing(20)
                .align_items(iced::Alignment::Center),
            );
        }

        if let Some(ref graph) = self.course_graph {
            for (idx, set) in self.required_courses.iter().rev().enumerate() {
                if idx > 0 {
                    right = right.push(horizontal_rule(2));
                }
                for course in set.inner {
                    match graph.courses[course].ntype {
                        NodeType::Or => panic!(
                            "Logic error, the following node was in a `CourseSet`:\n{:?}",
                            graph.courses[course]
                        ),
                        NodeType::Course(ref course) => {
                            right = right.push(text(course.id.clone()));
                        }
                    }
                }
            }
        }

        let main_content = Split::new(
            left,
            right,
            self.ui_states.main_divider_pos,
            split::Axis::Vertical,
            Message::MainDividerResize,
        );

        let overlay = self.ui_states.error_modal.front().map(|err_msg| {
            Card::new(text("Error"), text(err_msg))
                .foot(
                    container(button("Ok").on_press(Message::ClearError))
                        .width(Length::Fill)
                        .align_x(iced::alignment::Horizontal::Right),
                )
                .max_width(250.0)
        });

        modal(main_content, overlay).into()
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::Light
    }
}
