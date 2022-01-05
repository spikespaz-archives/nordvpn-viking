use gtk::prelude::*;
// use gtk::{Application, ApplicationWindow};

fn build_ui(app: &gtk::Application) {
    let header_left_box = gtk::Box::default();
    header_left_box.append(
        &gtk::ToggleButton::builder()
            .icon_name("dialog-information-symbolic")
            .build(),
    );

    let header_middle_box = gtk::Box::default();
    header_middle_box.add_css_class("linked");

    let first_button = &gtk::ToggleButton::builder()
        .label("Connect")
        .active(true)
        .build();
    header_middle_box.append(first_button);
    header_middle_box.append(
        &gtk::ToggleButton::builder()
            .label("Account")
            .group(first_button)
            .build(),
    );
    header_middle_box.append(
        &gtk::ToggleButton::builder()
            .label("Settings")
            .group(first_button)
            .build(),
    );

    let header = gtk::HeaderBar::new();
    header.pack_start(&header_left_box);
    header.set_title_widget(Some(&header_middle_box));

    let window = gtk::ApplicationWindow::new(app);

    window.set_title(Some("NordVPN"));
    window.set_titlebar(Some(&header));

    window.set_default_size(900, 600);
    window.set_size_request(700, 500);

    window.present();
}

fn main() {
    let app = gtk::Application::new(Some("dev.birkett.nordvpn"), Default::default());
    app.connect_activate(build_ui);
    app.run();
}
