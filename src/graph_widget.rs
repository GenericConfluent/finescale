use iced::{widget::canvas::Cache, advanced::{Widget, widget::tree, layout, Renderer as _, renderer::Quad}, Renderer, application::StyleSheet, Size, Length, Background, Color, Element};

use crate::course_database::CourseDatabase;

// Think I can stick stuff in this to store widget state.
// Probably since I don't get a mut reference to the struct itself.
#[derive(Default)]
struct State;

pub struct GraphData {
    graph: CourseDatabase,
    cache: Cache,
}

#[derive(Default)]
pub struct GraphWidget <Theme = iced::Theme>
where
    Theme: StyleSheet
{
    style: Theme::Style
}

impl<'a, Message, Theme> From<GraphWidget<Theme>> for Element<'a, Message, Theme, Renderer>
where
    Theme: StyleSheet + 'a
{
    fn from(value: GraphWidget<Theme>) -> Self {
        Self::new(value)
    }
}

impl<Message, Theme> Widget<Message, Theme, Renderer> for GraphWidget<Theme>
where
    Theme: StyleSheet
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill
        }
    }

    fn layout(
        &self,
        _tree: &mut tree::Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(limits.max())
    }

    fn draw(
        &self,
        tree: &tree::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: layout::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        // Note: this exists
        let _state = tree.state.downcast_ref::<State>();

        // Draw a blue rectangle
        
        renderer.fill_quad(
            Quad { 
                bounds: iced::Rectangle::new(iced::Point::new(0.0, 0.0), Size::new(200.0, 200.0)), 
                border: iced::Border::default(), 
                shadow: iced::Shadow::default()
            }, 
            Background::Color(Color::new(0.0, 0.0, 1.0, 1.0)),
        );




    }
}
