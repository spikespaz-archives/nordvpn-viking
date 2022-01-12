use gtk::{gio, glib};

const RESOURCE_BYTES: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/assets/compiled.gresource"));

pub fn init() {
    let resource_data = glib::Bytes::from(&RESOURCE_BYTES);
    let resource = gio::Resource::from_data(&resource_data).unwrap();

    gio::resources_register(&resource);
}
