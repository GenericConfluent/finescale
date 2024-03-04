use iced::{window, Application, Settings};

fn main() -> iced::Result {
    finescale::FinescaleApp::run(Settings {
        id: Some("finescale".into()),
        window: window::Settings {
            size: iced::Size::new(800.0, 400.0),
            visible: true,
            decorations: true,
            ..Default::default()
        },
        antialiasing: true,
        ..Default::default()
    })
}
