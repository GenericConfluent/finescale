use iced::{window, Settings, Application};


fn main() -> iced::Result {
    finescale::FinescaleApp::run(Settings {
        id: Some("finescale".into()),
        window: window::Settings {
            size: iced::Size::new(800.0, 400.0),
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
