pub mod config;
pub mod entry;
pub mod menu_provider;

use crate::menu_provider::OpenWithMenuProvider;
use glib_sys::GType;
use gobject_sys::GTypeModule;
use libc::c_int;
use nautilus_extension::{nautilus_module, NautilusModule};

nautilus_module!(init);

fn init(module: *mut GTypeModule) -> GType {
    println!("OpenWithExtensionRs: {}", env!("CARGO_PKG_VERSION"));

    NautilusModule::new(module, "OpenWithExtensionRs")
        .add_menu_provider(OpenWithMenuProvider)
        .register()
}
