use graph_layout::core::format::ClipHandle;
use graph_layout::core::geometry::Point;
use graph_layout::core::style::StyleAttr;
use graph_layout::topo::layout::VisualGraph;
use iced::advanced::renderer::Quad;
use iced::advanced::widget::tree;
use iced::advanced::{layout, Renderer as _, Widget};
use iced::application::StyleSheet;
use iced::border::Radius;
use iced::{Color, Element, Length, Renderer, Size};

use crate::course_database::CourseDatabase;

// Think I can stick stuff in this to store widget state.
// Probably since I don't get a mut reference to the struct itself.
#[derive(Default)]
struct State;

enum QueueCommand {
    Rect { quad: Quad, color: Color },
}

// TODO: Would be good to add a field here so we can take errors.
struct GraphWidgetRenderContext<'a, Theme: StyleSheet> {
    graph_widget: &'a GraphWidget<Theme>,
    tree: &'a tree::Tree,
    renderer: &'a mut Renderer,
    theme: &'a Theme,
    style: &'a iced::advanced::renderer::Style,
    layout: layout::Layout<'a>,
    cursor: iced::advanced::mouse::Cursor,
    viewport: &'a iced::Rectangle,
}

#[derive(Default)]
pub struct GraphWidget<Theme = iced::Theme>
where
    Theme: StyleSheet,
{
    style: Theme::Style,
    course_graph: Option<CourseDatabase>,
}

trait IcedFrom<T> {
    fn iced_from(val: T) -> Self;
}

// TODO: Go to the layout-rs repo and fix this atrocity.
impl IcedFrom<graph_layout::core::color::Color> for iced::Color {
    fn iced_from(val: graph_layout::core::color::Color) -> Self {
        use iced::color;
        color!(val.to_web_color()[1..].parse::<u32>().unwrap())
    }
}

impl IcedFrom<Point> for iced::Size {
    fn iced_from(val: Point) -> Self {
        Self {
            width: val.x as f32,
            height: val.y as f32,
        }
    }
}

impl IcedFrom<Point> for iced::Point {
    fn iced_from(val: Point) -> Self {
        Self {
            x: val.x as f32,
            y: val.y as f32,
        }
    }
}

impl<Theme: StyleSheet> graph_layout::core::format::RenderBackend
    for GraphWidgetRenderContext<'_, Theme>
{
    fn draw_rect(&mut self, xy: Point, size: Point, look: &StyleAttr, clip: Option<ClipHandle>) {
        self.renderer.fill_quad(
            Quad {
                bounds: iced::Rectangle::new(
                    iced::Point::iced_from(xy),
                    iced::Size::iced_from(size),
                ),
                border: iced::Border {
                    radius: clip
                        .map(|radius_px| Radius::from(radius_px as f32))
                        .unwrap_or_default(),
                    width: look.line_width as f32,
                    color: iced::Color::iced_from(look.line_color),
                },
                shadow: iced::Shadow::default(),
            },
            look.fill_color
                .map(iced::Color::iced_from)
                .unwrap_or(iced::Color::TRANSPARENT),
        );
    }

    fn draw_line(&mut self, start: Point, stop: Point, look: &StyleAttr) {}

    fn draw_circle(&mut self, xy: Point, size: Point, look: &StyleAttr) {}
    fn draw_text(&mut self, xy: Point, text: &str, look: &StyleAttr) {}
    fn draw_arrow(
        &mut self,
        path: &[(Point, Point)],
        dashed: bool,
        head: (bool, bool),
        look: &StyleAttr,
        text: &str,
    ) {
    }

    // Funny hack
    fn create_clip(&mut self, xy: Point, size: Point, rounded_px: usize) -> ClipHandle {
        rounded_px
    }
}

impl<'a, Message, Theme> From<GraphWidget<Theme>> for Element<'a, Message, Theme, Renderer>
where
    Theme: StyleSheet + 'a,
{
    fn from(value: GraphWidget<Theme>) -> Self {
        Self::new(value)
    }
}

impl<Message, Theme> Widget<Message, Theme, Renderer> for GraphWidget<Theme>
where
    Theme: StyleSheet,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State)
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
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
        // `VisualGraph::do_it` panics when the graph is empty.
        let Some(ref cg) = self.course_graph else {
            return;
        };

        let mut ctx = GraphWidgetRenderContext {
            graph_widget: self,
            tree,
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        };
        let mut vg = VisualGraph::new(graph_layout::core::base::Orientation::TopToBottom);
        vg.do_it(false, false, false, &mut ctx);
    }
}
