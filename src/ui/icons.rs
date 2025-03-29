use iced::alignment;
use iced::font::Font;
use iced::widget::{text, Text};

/// See https://fonts.google.com/icons. Just find some online icon font editor
/// to modify the ttf file.
pub enum Icon {
    AccountTree,
    FullStackedBarChart,
    SideNavigation,
    DragIndicator,
    Settings,
    DeleteForever,
}

impl Icon {
    pub fn bytes() -> &'static [u8] {
        include_bytes!("../fonts/icons.ttf").as_slice()
    }
}

impl From<Icon> for Text<'static> {
    fn from(val: Icon) -> Self {
        let ch = match val {
            Icon::AccountTree => "A",
            Icon::FullStackedBarChart => "B",
            Icon::SideNavigation => "C",
            Icon::DragIndicator => "D",
            Icon::Settings => "E",
            Icon::DeleteForever => "F",
        };
        text(ch)
            .font(Font::with_name("icons"))
            .width(20)
            .horizontal_alignment(alignment::Horizontal::Center)
    }
}
