use gtk::prelude::*;
use super::stacks::StartView;

pub struct Window {
    pub widget: libhandy::ApplicationWindow,
}

impl Window {
    pub fn new() -> Self {
        let builder = gtk::Builder::from_resource("/com/github/Latesil/file-converter/window.ui");
        let widget: libhandy::ApplicationWindow = builder
            .get_object("main_window")
            .expect("Failed to find the window object");

        let main_stack: gtk::Stack = builder.get_object("main_stack").expect("Unable to get main stack");
        let start_view = StartView::new();

        main_stack.add(&start_view.widget);

        Self { widget }
    }
}
