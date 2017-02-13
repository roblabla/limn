extern crate limn;
extern crate glutin;
extern crate cassowary;

mod util;

use cassowary::strength::*;

use limn::widget::{EventHandler, EventArgs};
use limn::widget::builder::WidgetBuilder;
use limn::widgets::primitives::{self, RectStyle, RectStyleField};
use limn::widgets::drag::DragEvent;
use limn::widget::style::Value;
use limn::event::EventId;
use limn::event::id::*;
use limn::util::Dimensions;

struct DragHandler {
    start_pos: f64,
}
impl DragHandler {
    pub fn new() -> Self {
        DragHandler { start_pos: 0.0 }
    }
}
impl EventHandler for DragHandler {
    fn event_id(&self) -> EventId {
        WIDGET_DRAG
    }
    fn handle_event(&mut self, args: EventArgs) {
        let EventArgs { data, solver, layout, .. } = args;
        let &(ref drag_event, pos) = data.downcast_ref::<(DragEvent, (i32, i32))>().unwrap();
        let drag_pos = pos.0 as f64;
        match *drag_event {
            DragEvent::DragStart => {
                self.start_pos = drag_pos - solver.get_value(layout.left);
            },
            _ => {
                solver.update_solver(|solver| {
                    if !solver.has_edit_variable(&layout.left) {
                        solver.add_edit_variable(layout.left, STRONG).unwrap();
                    }
                    solver.suggest_value(layout.left, drag_pos - self.start_pos).unwrap();
                });
            }
        }
    }
}

fn main() {
    let (window, ui, event_queue) = util::init_default("Limn slider demo");
    util::load_default_font();

    let mut root_widget = WidgetBuilder::new();
    root_widget.layout.dimensions(Dimensions {
        width: 300.0,
        height: 300.0,
    });

    let rect_color = [0.1, 0.1, 0.1, 1.0];
    let style = RectStyle::from(vec!{RectStyleField::BackgroundColor(Value::Single(rect_color))});
    let mut slider_container = WidgetBuilder::new().set_drawable(primitives::rect_drawable(style));
    slider_container.layout.dimensions(Dimensions {
        width: 200.0,
        height: 30.0,
    });
    slider_container.layout.align_top(&root_widget, Some(10.0));
    slider_container.layout.center_horizontal(&root_widget);

    let rect_color = [0.4, 0.4, 0.4, 1.0];
    let style = RectStyle::from(vec!{RectStyleField::BackgroundColor(Value::Single(rect_color))});
    let mut slider = WidgetBuilder::new()
        .set_drawable(primitives::rect_drawable(style))
        .draggable()
        .add_handler(DragHandler::new());
    slider.layout.dimensions(Dimensions {
        width: 30.0,
        height: 30.0,
    });
    slider.layout.align_top(&root_widget, Some(10.0));

    slider_container.add_child(slider);
    root_widget.add_child(slider_container);

    util::set_root_and_loop(window, ui, root_widget, event_queue, vec!{});
}
