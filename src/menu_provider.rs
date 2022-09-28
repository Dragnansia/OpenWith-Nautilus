use crate::config::Config;
use glib::gobject_ffi::GObject;
use glib_sys::gpointer;
use gtk_sys::GtkWidget;
use nautilus_extension::{nautilus_menu_item_activate_cb, FileInfo, Menu, MenuItem, MenuProvider};
use std::path::Path;

pub struct OpenWithMenuProvider;

impl MenuProvider for OpenWithMenuProvider {
    fn get_file_items(
        &self,
        _window: *mut GtkWidget,
        files: &[nautilus_extension::FileInfo],
    ) -> Vec<nautilus_extension::MenuItem> {
        self.get_menu_item_list(files)
    }

    fn get_background_items(
        &self,
        _window: *mut GtkWidget,
        current_folder: &nautilus_extension::FileInfo,
    ) -> Vec<nautilus_extension::MenuItem> {
        self.get_menu_item_list(&vec![current_folder.clone()])
    }
}

impl OpenWithMenuProvider {
    fn get_menu_item_list(&self, files: &[FileInfo]) -> Vec<nautilus_extension::MenuItem> {
        let mut top_menu_item =
            MenuItem::new("OpenTopMenuItem", "Open with", "Open with something", None);
        let mut sub_items: Vec<MenuItem> = Vec::new();

        let config = Config::new();
        if config.entries.is_empty() {
            return vec![];
        }

        let dirs_count = files
            .iter()
            .filter(|f| {
                let uri = f.get_uri();
                let split_uri = uri.split("//").collect::<Vec<&str>>();
                let path = split_uri.last().unwrap_or(&"");

                Path::new(path).is_dir()
            })
            .collect::<Vec<&FileInfo>>()
            .len();

        config.entries.iter().for_each(|e| {
            let mut menu_item = MenuItem::new(
                e.name.clone(),
                e.label.clone().unwrap_or(e.name.clone()),
                e.tip.clone().unwrap_or_default(),
                None,
            );

            if e.only_folder.unwrap_or_default()
                && dirs_count > 0
                && e.max_folder.unwrap_or_default() as usize >= dirs_count
            {
                menu_item.set_activate_cb(run_command_cb);

                sub_items.push(menu_item);
                return;
            }
        });

        if sub_items.is_empty() {
            vec![]
        } else {
            let menu = Menu::new(&sub_items);
            top_menu_item.set_submenu(&menu);
            vec![top_menu_item]
        }
    }
}

nautilus_menu_item_activate_cb!(run_command_cb, run_command);

fn run_command(files: Vec<FileInfo>) {
    println!("cb");
}
