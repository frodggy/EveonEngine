use std::fs;

use dock::tab::{Dock, DockContext};
use egui_dock::{DockArea, Style};
use egui_glfw_gl::egui as eglfw;
use eveon_engine::{get_project, logger};

pub mod dock;
pub mod images;

#[derive(Debug)]
enum AssestType {
    File,
    Dir,
}

#[derive(Debug)]
struct Assest {
    pub path: String,
    pub assest_type: AssestType,
}

impl DockContext {
    pub fn assest(&self, ui: &mut eglfw::Ui) {
        let mut assests: Vec<Assest> = vec![];
        let project = get_project(self.project_path.clone());

        let res = fs::read_dir(project.get_resources_dir().to_str().unwrap());

        if let Ok(paths) = res {
            for path in paths {
                let path = path.unwrap();
                let md = path.metadata().unwrap();
                let r#type: AssestType = if md.is_dir() {
                    AssestType::Dir
                } else if md.is_file() {
                    AssestType::File
                } else {
                    panic!("")
                };

                assests.push(Assest {
                    path: format!("{}", path.path().to_str().unwrap()),
                    assest_type: r#type,
                });
            }
        }

        for asset in &assests {
            let r#type = match asset.assest_type {
                AssestType::Dir => "dir".to_string(),
                AssestType::File => "file".to_string(),
            };

            let _ = &ui.button(format!("{}: {}", asset.path, r#type));
        }
    }

    pub fn debug_console(&mut self, ui: &mut eglfw::Ui) {
        logger::logger_ui(ui);
    }
}

pub fn create_dock_and_windows(ctx: &eglfw::Context, project_path: String) {
    let mut tabs_tree: Dock = Dock::default();
    tabs_tree.ctx.project_path = project_path;

    DockArea::new(&mut tabs_tree.tree)
        .style(Style::from_egui(ctx.style().as_ref()))
        .show_close_buttons(tabs_tree.ctx.show_close_buttons)
        .draggable_tabs(tabs_tree.ctx.draggable_tabs)
        .show_tab_name_on_hover(tabs_tree.ctx.show_tab_name_on_hover)
        .show_add_buttons(tabs_tree.ctx.show_add_buttons)
        .show(&ctx, &mut tabs_tree.ctx);
}
