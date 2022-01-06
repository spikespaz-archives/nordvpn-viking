mod window;

use gtk::prelude::*;
use window::VikingApplicationWindow;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.spikespaz.nordvpn-viking"),
        Default::default(),
    );

    application.connect_activate(|app| {
        let win = VikingApplicationWindow::new(app);
        win.show();
    });
    application.run();
}
