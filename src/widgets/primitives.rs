use graphics;
use graphics::types::Color;

use widget::{Drawable, WidgetStyle, StyleArgs, DrawArgs};
use widget::style::Value;
use theme::STYLE_RECT;
use util::Scalar;

pub fn rect_drawable(style: RectStyle) -> Drawable {
    let draw_state = RectDrawState {
        background_color: style.background_color.default(),
        corner_radius: style.corner_radius.default(),
    };
    let mut drawable = Drawable::new(draw_state, draw_rect);
    drawable.style = Some(WidgetStyle::new(style, apply_rect_style));
    drawable
}

pub struct RectDrawState {
    pub background_color: Color,
    pub corner_radius: Option<Scalar>,
}
#[derive(Clone)]
pub struct RectStyle {
    pub background_color: Value<Color>,
    pub corner_radius: Value<Option<Scalar>>,
}
pub enum RectStyleField {
    BackgroundColor(Value<Color>),
    CornerRadius(Value<Option<Scalar>>),
}
impl RectStyle {
    pub fn from(fields: Vec<RectStyleField>) -> Self {
        let mut style = STYLE_RECT.clone();
        style.extend(fields);
        style
    }
    pub fn extend(&mut self, mut style: Vec<RectStyleField>) {
        for field in style.drain(..) {
            match field {
                RectStyleField::BackgroundColor(val) => self.background_color = val,
                RectStyleField::CornerRadius(val) => self.corner_radius = val,
            }
        }
    }
}
use std::f64::consts::PI;
use util::{Rectangle, Point};
pub fn draw_rect(args: DrawArgs) {
    let DrawArgs { state, bounds, context, graphics, .. } = args;
    let state: &RectDrawState = state.downcast_ref().unwrap();
    if let Some(radius) = state.corner_radius {
        let points_per_corner = 8;
        let angle_per_step = 2.0 * PI / (points_per_corner * 4) as Scalar;
        fn circle_coords(radius: f64, step: f64, angle_per_step: f64) -> [f64; 2] {
            [radius * (step * angle_per_step).cos(), radius * (step * angle_per_step).sin()]
        };
        // corners are center points of four circle segments
        let inner_rect = Rectangle {
            left: bounds.left + radius,
            top: bounds.top + radius,
            width: bounds.width - 2.0 * radius,
            height: bounds.height - 2.0 * radius,
        };
        let points: Vec<[f64; 2]> = (0..4).flat_map(|corner| {
            let center: Point = match corner {
                0 => inner_rect.bottom_right(),
                1 => inner_rect.bottom_left(),
                2 => inner_rect.top_left(),
                3 => inner_rect.top_right(),
                _ => unreachable!(),
            };
            let step_offset: u32 = corner * points_per_corner;
            (0..points_per_corner + 1).map(move |corner_step| {
                let circle_step = step_offset + corner_step;
                let circle_offset = circle_coords(radius, circle_step as f64, angle_per_step);
                [center.x + circle_offset[0], center.y + circle_offset[1]]
            })
        }).collect();
        graphics::Polygon::new(state.background_color)
            .draw(&points, &context.draw_state, context.transform, graphics);
    } else {
        graphics::Rectangle::new(state.background_color)
            .draw(bounds, &context.draw_state, context.transform, graphics);
    }
}

pub fn apply_rect_style(args: StyleArgs) {
    let state: &mut RectDrawState = args.state.downcast_mut().unwrap();
    let style: &RectStyle = args.style.downcast_ref().unwrap();
    state.background_color = style.background_color.from_props(&args.props);
    state.corner_radius = style.corner_radius.from_props(&args.props);
}

pub fn ellipse_drawable(background_color: Color, border: Option<graphics::ellipse::Border>) -> Drawable {
    let draw_state = EllipseDrawState { background_color: background_color, border: border };
    Drawable::new(draw_state, draw_ellipse)
}
pub struct EllipseDrawState {
    pub background_color: Color,
    pub border: Option<graphics::ellipse::Border>,
}
pub fn draw_ellipse(draw_args: DrawArgs) {
    let DrawArgs { state, bounds, context, graphics, .. } = draw_args;
    let state: &EllipseDrawState = state.downcast_ref().unwrap();

    graphics::Ellipse::new(state.background_color)
        .maybe_border(state.border)
        .draw(bounds, &context.draw_state, context.transform, graphics);
}
