use gettextrs::*;
use gio::prelude::*;
use gtk::prelude::*;

mod config;
mod window;
mod stacks;
use crate::window::Window;

#[macro_use]
extern crate gtk_macros;

fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
    libhandy::init();

    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain("file-converter", config::LOCALEDIR);
    textdomain("file-converter");

    let res = gio::Resource::load(config::PKGDATADIR.to_owned() + "/file-converter.gresource")
        .expect("Could not load resources");
    gio::resources_register(&res);

    let app = gtk::Application::new(Some("com.github.Latesil.file-converter"), Default::default()).unwrap();
    app.connect_activate(move |app| {
        let window = Window::new();

        window.widget.set_application(Some(app));
        app.add_window(&window.widget);
        window.widget.present();
    });

    let ret = app.run(&std::env::args().collect::<Vec<_>>());
    std::process::exit(ret);
}
