use gtk_sys::GtkWidget;
use nautilus_extension::{FileInfo, Menu, MenuItem, MenuProvider};

pub struct OpenWithMenuProvider {}

impl MenuProvider for OpenWithMenuProvider {
    fn get_file_items(
        &self,
        window: *mut GtkWidget,
        files: &[nautilus_extension::FileInfo],
    ) -> Vec<nautilus_extension::MenuItem> {
        self.get_menu_item_list()
    }

    fn get_background_items(
        &self,
        window: *mut GtkWidget,
        current_folder: &nautilus_extension::FileInfo,
    ) -> Vec<nautilus_extension::MenuItem> {
        self.get_menu_item_list()
    }
}

impl OpenWithMenuProvider {
    fn get_menu_item_list(&self) -> Vec<nautilus_extension::MenuItem> {
        let mut top_menu_item =
            MenuItem::new("OpenTopMenuItem", "Open with", "Open with something", None);

        let mut sub_items: Vec<MenuItem> = Vec::new();
        sub_items.push(MenuItem::new(
            "bbbbbbbbbbbbbbbbbbbb",
            "bbbbbbbbbbbbbbbbbbbb",
            "bbbbbbbbbbbbbbbbbbbb",
            None,
        ));
        sub_items.push(MenuItem::new(
            "aaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaa",
            None,
        ));

        let menu = Menu::new(&sub_items);
        top_menu_item.set_submenu(&menu);
        vec![top_menu_item]
    }
}
