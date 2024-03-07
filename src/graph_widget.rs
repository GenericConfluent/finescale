use graph_layout::core::format::ClipHandle;
use graph_layout::core::geometry::Point;
use graph_layout::core::style::StyleAttr;
use graph_layout::topo::layout::VisualGraph;
use iced::advanced::renderer::Quad;
use iced::application::StyleSheet;
use iced::widget::canvas::{self};
use iced::widget::text::LineHeight;
use iced::{Color, Size};

use crate::course_database::CourseDatabase;

// Think I can stick stuff in this to store widget state.
// Probably since I don't get a mut reference to the struct itself.
#[derive(Default)]
struct State;

enum QueueCommand {
    Rect { quad: Quad, color: Color },
}

// TODO: Would be good to add a field here so we can take errors.
struct GraphWidgetRenderContext<'a> {
    f: &'a mut canvas::Frame,
}

#[derive(Default)]
pub struct GraphWidget<Theme = iced::Theme>
where
    Theme: StyleSheet,
{
    style: Theme::Style,
    cache: canvas::Cache,
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

impl IcedFrom<&StyleAttr> for canvas::Stroke<'_> {
    fn iced_from(val: &StyleAttr) -> Self {
        Self::default()
            .with_color(iced::Color::iced_from(val.line_color))
            .with_width(val.line_width as f32)
    }
}

impl graph_layout::core::format::RenderBackend for GraphWidgetRenderContext<'_> {
    fn draw_rect(&mut self, xy: Point, size: Point, look: &StyleAttr, clip: Option<ClipHandle>) {
        let top_left = iced::Point::iced_from(xy);
        let size = Size::iced_from(size);
        if let Some(colour) = look.fill_color {
            self.f
                .fill_rectangle(top_left, size, iced::Color::iced_from(colour));
        }
        self.f.stroke(
            &canvas::Path::rectangle(top_left, size),
            canvas::Stroke::iced_from(look),
        );
    }

    fn draw_line(&mut self, start: Point, stop: Point, look: &StyleAttr) {
        let start = iced::Point::iced_from(start);
        let stop = iced::Point::iced_from(stop);
        self.f.stroke(
            &canvas::Path::line(start, stop),
            canvas::Stroke::iced_from(look),
        );
    }

    /// `layout-rs` uses this function to draw ellipses which is why there is a
    /// size parameter. This is a simple implementation so it'll just draw a
    /// circle sufficiently big that it would contain the intended ellipse.
    fn draw_circle(&mut self, xy: Point, size: Point, look: &StyleAttr) {
        self.f.stroke(
            &canvas::Path::circle(
                iced::Point::iced_from(xy),
                f64::max(size.x, size.y) as f32 / 2.0,
            ),
            canvas::Stroke::iced_from(look),
        );
    }

    fn draw_text(&mut self, xy: Point, text: &str, look: &StyleAttr) {
        self.f.fill_text(canvas::Text {
            content: text.into(),
            position: iced::Point::iced_from(xy),
            color: iced::Color::iced_from(look.line_color),
            size: iced::Pixels(look.font_size as f32),
            line_height: LineHeight::Relative(1.0),
            font: iced::Font::DEFAULT,
            horizontal_alignment: iced::alignment::Horizontal::Center,
            vertical_alignment: iced::alignment::Vertical::Center,
            shaping: iced::widget::text::Shaping::Basic,
        });
    }

    /// `GraphWidget` should not need to draw text to distinguish between
    /// a prereq and a coreq, so `text` can be ignored.
    // https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths#curve_commands
    // https://docs.rs/layout-rs/latest/src/layout/backends/svg.rs.html#227
    fn draw_arrow(
        &mut self,
        path: &[(Point, Point)],
        dashed: bool,
        head: (bool, bool),
        look: &StyleAttr,
        text: &str,
    ) {
        fn reflect_control(control_b: iced::Point, to: iced::Point) -> iced::Point {
            iced::Point {
                x: 2.0 * to.x - control_b.x,
                y: 2.0 * to.y - control_b.y,
            }
        }

        let mut path_iter = path.iter().copied().map(|(control2, line_end)| {
            (
                iced::Point::iced_from(control2),
                iced::Point::iced_from(line_end),
            )
        });
        let stroke = canvas::Stroke {
            line_dash: if dashed {
                canvas::LineDash {
                    segments: &[1.0, 1.0],
                    offset: 0,
                }
            } else {
                canvas::LineDash::default()
            },
            ..canvas::Stroke::iced_from(look)
        };
        // use a Builder to add points manually
        self.f.stroke(
            &canvas::Path::new(|p| {
                // "Handle the 'exit vector' from the first point"
                let first = path_iter.next().unwrap();
                let second = path_iter.next().unwrap();
                p.move_to(first.0);
                p.bezier_curve_to(first.1, second.0, second.1);

                // "Handle the 'entry vector' from the rest of the points"
                // Here we need to mimic the svg S command and mirror control_b
                // to use it as the next control_a
                let mut control_a = reflect_control(second.0, second.1);
                for (control_b, to) in path_iter {
                    p.bezier_curve_to(control_a, control_b, to);
                    control_a = reflect_control(control_b, to);
                }
            }),
            stroke,
        );
    }

    // Funny hack
    fn create_clip(&mut self, xy: Point, size: Point, rounded_px: usize) -> ClipHandle {
        rounded_px
    }
}

impl<Message, Theme> canvas::Program<Message, Theme> for GraphWidget<Theme>
where
    Theme: StyleSheet,
{
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        theme: &Theme,
        bounds: iced::Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        // `VisualGraph::do_it` panics when the graph is empty.
        let Some(ref cg) = self.course_graph else {
            return vec![];
        };

        let mut vg = VisualGraph::new(graph_layout::core::base::Orientation::TopToBottom);

        self.cache.draw(renderer, bounds.size(), move |frame| {
            let mut ctx = GraphWidgetRenderContext { f: frame };
            vg.do_it(false, false, false, &mut ctx);
        });
        vec![]
    }

    fn update(
        &self,
        _state: &mut Self::State,
        _event: canvas::Event,
        _bounds: iced::Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        (canvas::event::Status::Ignored, None)
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: iced::Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> iced::advanced::mouse::Interaction {
        iced::advanced::mouse::Interaction::default()
    }
}
