use egui_backend::egui;
use egui_dock::{self, NodeIndex, Tree};
use egui_glfw_gl as egui_backend;

pub struct DockContext {
    pub project_path: String,
    pub show_close_buttons: bool,
    pub draggable_tabs: bool,
    pub show_tab_name_on_hover: bool,
    pub show_add_buttons: bool,
}

impl egui_dock::TabViewer for DockContext {
    type Tab = String;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab.as_str() {
            "Debug Console" => self.debug_console(ui),
            _ => {
                ui.label(format!("Content of {tab}"));
            }
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }
}

pub struct Dock {
    pub tree: Tree<String>,
    pub ctx: DockContext,
}

impl Default for Dock {
    fn default() -> Self {
        let mut tree = Tree::new(vec!["tab1".to_owned(), "tab2".to_owned()]);

        // You can modify the tree before constructing the dock
        let [a, b] = tree.split_left(NodeIndex::root(), 0.3, vec!["tab3".to_owned()]);
        let [_, _] = tree.split_below(
            a,
            0.7,
            vec!["Assests".to_owned(), "Debug Console".to_owned()],
        );
        let [_, _] = tree.split_below(b, 0.5, vec!["tab5".to_owned()]);

        let ctx = DockContext {
            project_path: String::new(),
            show_close_buttons: true,
            draggable_tabs: false,
            show_tab_name_on_hover: true,
            show_add_buttons: false,
        };

        Self { tree, ctx }
    }
}
