use gtk::prelude::{BoxExt, ButtonExt, DialogExt, GtkWindowExt, ToggleButtonExt, WidgetExt};
use relm4::{send, AppUpdate, ComponentUpdate, Model, RelmApp, RelmComponent, Sender, Widgets};

const DEFAULT_SIZE: Size = Size { x: 800, y: 600 };
const MINIMUM_SIZE: Size = Size { x: 600, y: 400 };

#[derive(Clone, Copy)]
struct Size {
    x: i32,
    y: i32,
}

impl Size {
    fn new(x: i32, y: i32) -> Self {
        Size { x, y }
    }
}

enum HeaderMsg {
    Connect,
    Account,
    Settings,
    OpenInfoDialog,
}

struct HeaderModel {}

impl Model for HeaderModel {
    type Msg = HeaderMsg;
    type Widgets = HeaderWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for HeaderModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        HeaderModel {}
    }

    fn update(
        &mut self,
        msg: HeaderMsg,
        _components: &(),
        _sender: Sender<HeaderMsg>,
        parent_sender: Sender<AppMsg>,
    ) {
        match msg {
            HeaderMsg::Connect => {
                send!(parent_sender, AppMsg::SetMode(AppMode::Connect));
            }
            HeaderMsg::Account => {
                send!(parent_sender, AppMsg::SetMode(AppMode::Account));
            }
            HeaderMsg::Settings => {
                send!(parent_sender, AppMsg::SetMode(AppMode::Settings));
            }
            HeaderMsg::OpenInfoDialog => {}
        }
    }
}

#[relm4_macros::widget]
impl Widgets<HeaderModel, AppModel> for HeaderWidgets {
    view! {
        gtk::HeaderBar {
            pack_start = &gtk::Box {
                append = &gtk::Button {
                    set_icon_name: "dialog-information-symbolic",
                    connect_clicked(sender) => move |_| {
                        send!(sender, HeaderMsg::OpenInfoDialog);
                    },
                },
            },
            set_title_widget = Some(&gtk::Box) {
                append = &gtk::Box {
                    add_css_class: "linked",
                    append: group = &gtk::ToggleButton {
                        set_label: "Connect",
                        set_active: true,
                        connect_toggled(sender) => move |btn| {
                            if btn.is_active() {
                                send!(sender, HeaderMsg::Connect);
                            }
                        },
                    },
                    append = &gtk::ToggleButton {
                        set_label: "Account",
                        set_group: Some(&group),
                        connect_toggled(sender) => move |btn| {
                            if btn.is_active() {
                                send!(sender, HeaderMsg::Account);
                            }
                        },
                    },
                    append = &gtk::ToggleButton {
                        set_label: "Settings",
                        set_group: Some(&group),
                        connect_toggled(sender) => move |btn| {
                            if btn.is_active() {
                                send!(sender, HeaderMsg::Settings);
                            }
                        },
                    },
                },
            }
        }
    }
}

struct DialogModel {
    hidden: bool,
}

enum DialogMsg {
    Show,
    Accept,
    Cancel,
}

impl Model for DialogModel {
    type Msg = DialogMsg;
    type Widgets = DialogWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for DialogModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        DialogModel { hidden: true }
    }

    fn update(
        &mut self,
        msg: DialogMsg,
        _components: &(),
        _sender: Sender<DialogMsg>,
        parent_sender: Sender<AppMsg>,
    ) {
        match msg {
            DialogMsg::Show => self.hidden = false,
            DialogMsg::Accept => {
                self.hidden = true;
                send!(parent_sender, AppMsg::Close);
            }
            DialogMsg::Cancel => self.hidden = true,
        }
    }
}

#[relm4_macros::widget]
impl Widgets<DialogModel, AppModel> for DialogWidgets {
    view! {
        dialog = gtk::MessageDialog {
            set_transient_for: parent!(Some(&parent_widgets.main_window)),
            set_modal: true,
            set_visible: watch!(!model.hidden),
            set_text: Some("Do you want to close before saving?"),
            set_secondary_text: Some("All unsaved changes will be lost"),
            add_button: args!("Close", gtk::ResponseType::Accept),
            add_button: args!("Cancel", gtk::ResponseType::Cancel),
            connect_response(sender) => move |_, resp| {
                send!(sender, if resp == gtk::ResponseType::Accept {
                    DialogMsg::Accept
                } else {
                    DialogMsg::Cancel
                });
            }
        }
    }
}

#[derive(relm4_macros::Components)]
struct AppComponents {
    header: RelmComponent<HeaderModel, AppModel>,
    dialog: RelmComponent<DialogModel, AppModel>,
}

#[derive(Debug)]
enum AppMode {
    Connect,
    Account,
    Settings,
}

enum AppMsg {
    SetMode(AppMode),
    CloseRequest,
    Close,
}

struct AppModel {
    mode: AppMode,
    unsaved: bool,
    size: Option<Size>,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

#[relm4_macros::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        main_window = gtk::ApplicationWindow {
            set_size_request: args!(MINIMUM_SIZE.x, MINIMUM_SIZE.y),
            set_default_width: model.size.unwrap_or(DEFAULT_SIZE).x,
            set_default_height: model.size.unwrap_or(DEFAULT_SIZE).y,
            set_titlebar: Some(components.header.root_widget()),
            set_child = Some(&gtk::Label) {
                set_label: watch!(&format!("Placeholder for {:?}", model.mode)),
            },
            connect_fullscreened_notify(sender) => move |_| {},
            connect_maximized_notify(sender) => move |_| {},
            // connect_resize
            connect_close_request(sender) => move |_| {
                send!(sender, AppMsg::CloseRequest);
                gtk::Inhibit(true)
            }
        }
    }
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, components: &AppComponents, _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::SetMode(mode) => {
                self.mode = mode;
            }
            AppMsg::CloseRequest => {
                if self.unsaved {
                    components.dialog.send(DialogMsg::Show).unwrap();
                } else {
                    return false;
                }
            }
            AppMsg::Close => {
                return false;
            }
        }
        true
    }
}

fn main() {
    let model = AppModel {
        mode: AppMode::Connect,
        unsaved: false,
        size: None,
    };
    let relm = RelmApp::new(model);
    relm.run();
}
