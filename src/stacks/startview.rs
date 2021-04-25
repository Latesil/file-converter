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

        get_widget!(builder, gtk::Label, start_label);

        builder.connect_signals(|builder, handler_name| {
            match handler_name {
                "on_add_button_clicked" => Box::new(self::StartView::add_button_clicked),
                _ => Box::new(|_| {None})
            }
        });

        Self { widget }
    }

    fn add_button_clicked(param: &[glib::Value]) -> Option<glib::Value>  {
        println!("on_add_button_clicked");
        None
    }
}
