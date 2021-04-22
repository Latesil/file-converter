use gtk::prelude::*;
use gio::prelude::*;
use gtk::subclass::prelude::*;

pub struct StartView {
    pub widget: gtk::Stack,
}

impl StartView {
    pub fn new() -> Self {
        let builder = gtk::Builder::from_resource("/com/github/Latesil/file-converter/startview.ui");
        let widget: gtk::Stack = builder
            .get_object("start_view")
            .expect("Failed to find the start_view object");

        Self { widget }
    }
}
