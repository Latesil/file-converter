use gtk::prelude::*;
use gio::prelude::*;
use gtk::subclass::prelude::*;

pub struct FileListView {
    pub widget: gtk::Stack,
}

impl FileListView {
    pub fn new() -> Self {
        let builder = gtk::Builder::from_resource("/com/github/Latesil/file-converter/filelistview.ui");
        let widget: gtk::Stack = builder
            .get_object("file_list_view")
            .expect("Failed to find the file_list_view object");

        get_widget!(builder, gtk::Label, file_list_label);

        Self { widget }
    }
}