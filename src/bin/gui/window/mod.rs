mod imp;

use gtk::{gio, glib};

glib::wrapper! {
    pub struct VikingApplicationWindow(ObjectSubclass<imp::VikingApplicationWindow>)
    @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
    @implements gio::ActionMap, gio::ActionGroup;
}

impl VikingApplicationWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)])
            .expect("Failed to create `VikingApplicationWindow`")
    }
}
