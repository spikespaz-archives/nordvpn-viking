use gtk::prelude::*;

fn main() {
    let app = gtk::Application::new(Some("dev.birkett.nordvpn"), Default::default());
    app.connect_activate(|app| {
        let builder = gtk::Builder::from_string(include_str!("ui/window.ui"));
        let window: gtk::ApplicationWindow = builder
            .object("window")
            .expect("Could not get object `window` from builder.");

        window.set_application(Some(app));
        window.present();
    });
    app.run();
}
