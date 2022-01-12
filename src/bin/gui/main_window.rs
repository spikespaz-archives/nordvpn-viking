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

mod imp {
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{glib, CompositeTemplate};

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/spikespaz/nordvpn-viking/ui/application_window.ui")]
    pub struct VikingApplicationWindow {}

    #[glib::object_subclass]
    impl ObjectSubclass for VikingApplicationWindow {
        const NAME: &'static str = "VikingApplicationWindow";
        type Type = super::VikingApplicationWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(class: &mut Self::Class) {
            Self::bind_template(class);
            // UtilityCallbacks::bind_template_callbacks(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for VikingApplicationWindow {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for VikingApplicationWindow {}
    impl WindowImpl for VikingApplicationWindow {}
    impl ApplicationWindowImpl for VikingApplicationWindow {}

    // struct UtilityCallbacks {}

    // #[gtk::template_callbacks(functions)]
    // impl UtilityCallbacks {}
}
