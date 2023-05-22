use iced::{
    executor, theme::Palette, window, Application, Color, Command, Element, Settings, Theme,
};

// Some graph of all courses
#[derive(Debug)]
struct CourseData;

#[derive(Default)]
struct FinescaleApp {
    course_data: Option<CourseData>,
}

#[derive(Debug)]
enum Message {
    LoadedCourses(CourseData),
}

async fn load_courses<P: AsRef<std::path::Path>>(path: P) -> CourseData {
    let reader = std::fs::File::open(path).unwrap();
    let _json: serde_json::Value = serde_json::from_reader(reader).unwrap();
    CourseData
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
        "Hello, world".into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::custom(Palette {
            background: Color::from_rgba8(14, 14, 14, 0.1),
            text: Color::WHITE,
            primary: Color::WHITE,
            success: Color::WHITE,
            danger: Color::WHITE,
        })
    }
}

fn main() -> iced::Result {
    FinescaleApp::run(Settings {
        id: Some("finescale".into()),
        window: window::Settings {
            size: (800, 400),
            position: window::Position::Centered,
            visible: true,
            decorations: false,
            transparent: true,
            ..Default::default()
        },
        ..Default::default()
    })
    // let reader = std::fs::File::open("data/courses.json").unwrap();
    // let _json: serde_json::Value = serde_json::from_reader(reader).unwrap();
    // let mut graph = Graph::<&str, &str>::new();

    // Ok(())
}
