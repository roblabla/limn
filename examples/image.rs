extern crate limn;

mod util;

use limn::widget::builder::WidgetBuilder;
use limn::widgets::image;

fn main() {
    let (mut window, ui) = util::init_default("Limn button demo");
    let image_id = util::load_default_image(&mut window);

    let mut root_widget = WidgetBuilder::new();
    let image_drawable = image::image_drawable(image_id);
    let image_dims = image::measure_image(&image_drawable);
    let mut image_widget = WidgetBuilder::new().set_drawable(image_drawable);
    image_widget.layout.dimensions(image_dims);
    image_widget.layout.center(&root_widget.layout);
    image_widget.layout.pad(50.0, &root_widget.layout);
    root_widget.add_child(Box::new(image_widget));

    util::set_root_and_loop(window, ui, root_widget);
}