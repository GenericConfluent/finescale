use iced::theme::Palette;
use iced::{executor, Application, Color, Command, Element, Theme};
use iced::widget::{text, column};

mod course_database;
use course_database::{CourseDatabase, CourseId};

mod graph_widget;

#[derive(Default)]
pub struct FinescaleApp {
    desired_courses: Vec<CourseId>,
    course_database: CourseDatabase,
}

#[derive(Default, Debug)]
pub struct CourseGraph;

#[derive(Debug)]
pub enum Message {
    LoadedCourses(CourseGraph),
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
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        column![
            text("Hello"),
            graph_widget::GraphWidget::default(),
        ].into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::custom("apptheme".to_string(), Palette {
            background: Color::from_rgba8(14, 14, 14, 0.1),
            text: Color::WHITE,
            primary: Color::WHITE,
            success: Color::WHITE,
            danger: Color::WHITE,
        })
    }
}

