use crate::config::Config;
use gtk_sys::GtkWidget;
use nautilus_extension::{Menu, MenuItem, MenuProvider};

pub struct OpenWithMenuProvider;

impl MenuProvider for OpenWithMenuProvider {
    fn get_file_items(
        &self,
        _window: *mut GtkWidget,
        _files: &[nautilus_extension::FileInfo],
    ) -> Vec<nautilus_extension::MenuItem> {
        self.get_menu_item_list()
    }

    fn get_background_items(
        &self,
        _window: *mut GtkWidget,
        _current_folder: &nautilus_extension::FileInfo,
    ) -> Vec<nautilus_extension::MenuItem> {
        self.get_menu_item_list()
    }
}

impl OpenWithMenuProvider {
    fn get_menu_item_list(&self) -> Vec<nautilus_extension::MenuItem> {
        let mut top_menu_item =
            MenuItem::new("OpenTopMenuItem", "Open with", "Open with something", None);
        let mut sub_items: Vec<MenuItem> = Vec::new();

        let config = Config::new();
        config.entries.iter().for_each(|e| {
            sub_items.push(MenuItem::new(
                e.name.clone(),
                e.label.clone().unwrap_or(e.name.clone()),
                e.tip.clone().unwrap_or_default(),
                None,
            ));

            // Add command bind
        });

        let menu = Menu::new(&sub_items);
        top_menu_item.set_submenu(&menu);
        vec![top_menu_item]
    }
}
