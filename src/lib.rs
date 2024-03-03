#![allow(dead_code, unused_variables)]

use iced::theme::Palette;
use iced::widget::{button, column, container, horizontal_rule, row, text, text_input};
use iced::{executor, Application, Color, Command, Element, Length, Theme};

use iced_aw::native::Split;
use iced_aw::{modal, split, Card};

mod course_database;
use course_database::{CourseDatabase, CourseId};

mod graph_widget;

#[derive(Default)]
pub struct FinescaleApp {
    desired_courses: Vec<CourseId>,
    course_database: CourseDatabase,
    ui_states: UiStates,
}

#[derive(Default)]
struct UiStates {
    error_modal: Option<String>,
    main_divider_pos: Option<u16>,
    course_input_val: String,
}

#[derive(Default, Debug, Clone)]
pub struct CourseGraph;

#[derive(Debug, Clone)]
pub enum Message {
    LoadedCourses(CourseGraph),
    MainDividerResize(u16),
    CourseInputEvent(String),
    CourseInputSubmit,
    ClearError,
}

async fn load_courses<P: AsRef<std::path::Path>>(path: P) -> CourseGraph {
    //let reader = std::fs::File::open(path).unwrap();
    //let _json: serde_json::Value = serde_json::from_reader(reader).unwrap();
    CourseGraph
}

impl Application for FinescaleApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            FinescaleApp::default(),
            Command::perform(load_courses("data/courses.json"), Message::LoadedCourses),
        )
    }

    fn title(&self) -> String {
        "Finescale".to_string()
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        match _message {
            Message::LoadedCourses(_) => {}
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
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut left = column![
            text("Desired Classes"),
            text_input("Start typing!", &self.ui_states.course_input_val)
                .on_input(Message::CourseInputEvent)
                .on_submit(Message::CourseInputSubmit),
        ];
        let mut right = column![row![text("Required Classes"),], horizontal_rule(2)];

        for course in self.desired_courses.iter() {
            right = right.push(text(course));
            left = left.push(text(course));
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
            Card::new(text("Error"), text(err_msg)).foot(
                container(button("Ok").on_press(Message::ClearError))
                    .width(Length::Fill)
                    .align_x(iced::alignment::Horizontal::Right),
            )
        });

        modal(main_content, overlay).into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::custom(
            "apptheme".to_string(),
            Palette {
                background: Color::from_rgba8(14, 14, 14, 0.1),
                text: Color::WHITE,
                primary: Color::WHITE,
                success: Color::WHITE,
                danger: Color::WHITE,
            },
        )
    }
}
