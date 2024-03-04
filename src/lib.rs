#![allow(dead_code, unused_variables)]

use std::rc::Rc;
use std::sync::Arc;

use iced::alignment::Horizontal;
use iced::theme::Palette;
use iced::widget::{button, column, container, horizontal_rule, row, text, text_input, Text};
use iced::{executor, font, Application, Color, Command, Element, Length, Padding, Theme};

use iced_aw::native::Split;
use iced_aw::{modal, split, Card, BOOTSTRAP_FONT_BYTES};

mod course_database;
use course_database::{CourseDatabase, CourseId};
use icons::Icon;

mod graph_widget;
mod icons;

#[derive(Default)]
pub struct FinescaleApp {
    desired_courses: Vec<CourseId>,
    course_database: Option<CourseDatabase>,
    ui_states: UiStates,
}

#[derive(Default)]
struct UiStates {
    error_modal: Option<String>,
    main_divider_pos: Option<u16>,
    course_input_val: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    LoadedCourses(Arc<anyhow::Result<CourseDatabase>>),
    MainDividerResize(u16),
    CourseInputEvent(String),
    CourseInputSubmit,
    IconsLoaded(Result<(), font::Error>),
    ClearError,
}

async fn load_courses<P: AsRef<std::path::Path>>(path: P) -> Arc<anyhow::Result<CourseDatabase>> {
    CourseDatabase::new("[]").into()
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
            Message::ClearError => self.ui_states.error_modal = None,
            // TODO: Limit the divider movement
            Message::MainDividerResize(amt) => self.ui_states.main_divider_pos = Some(amt),
            Message::CourseInputEvent(val) => self.ui_states.course_input_val = val,
            Message::CourseInputSubmit => {
                match self.ui_states.course_input_val.parse::<CourseId>() {
                    Ok(course) => {
                        self.desired_courses.push(course);
                        self.ui_states.course_input_val.clear();
                    }
                    Err(issue) => {
                        self.ui_states.error_modal = Some(issue.to_string());
                    }
                }
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
            row![text("Required Classes")
                .width(Length::Fill)
                .size(40)
                .style(Color::from_rgb(0.5, 0.5, 0.5))
                .horizontal_alignment(iced::alignment::Horizontal::Left),],
            horizontal_rule(2)
        ];

        for course in self.desired_courses.iter() {
            left = left.push(
                row![
                    text(course).width(Length::Fill),
                    button(Into::<Text>::into(Icon::DeleteForever)).padding(10)
                ]
                .spacing(20)
                .align_items(iced::Alignment::Center),
            );
            right = right.push(text(course));
        }

        // Todo read and push courses.
        let main_content = Split::new(
            left,
            right,
            self.ui_states.main_divider_pos,
            split::Axis::Vertical,
            Message::MainDividerResize,
        );

        let overlay = self.ui_states.error_modal.as_ref().map(|err_msg| {
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
