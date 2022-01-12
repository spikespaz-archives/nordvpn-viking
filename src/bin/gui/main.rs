mod resources;
mod window;

use gtk::prelude::*;
use window::VikingApplicationWindow;

fn main() {
    resources::init();

    let application = gtk::Application::new(
        Some("com.github.spikespaz.nordvpn-viking"),
        Default::default(),
    );

    application.connect_activate(|app| {
        let window = VikingApplicationWindow::new(app);

        window.show();
    });

    application.run();
}
