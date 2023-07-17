use egui_backend::egui::{vec2, Pos2, Rect};
use egui_dock::{DockArea, Style};
use egui_glfw_gl as egui_backend;
use egui_glfw_gl::egui;
use eveon_editor::dock::tab::Dock;
use eveon_editor::images::ImageLoader;
use eveon_engine::get_project;
use eveon_engine::{
    core::{graphics::shaders, graphics::window::*},
    logger,
};

use std::{env, mem, ptr};

fn main() {
    let path = env::args().last().unwrap();
    let project = get_project(path.clone());

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));

    let mut ewindow = EvWindow::new(&project.get_name(), 1080, 720);
    logger::init().unwrap();

    ewindow.window.set_char_polling(true);
    ewindow.window.set_cursor_pos_polling(true);
    ewindow.window.set_key_polling(true);
    ewindow.window.set_mouse_button_polling(true);

    gl::load_with(|s| ewindow.window.get_proc_address(s) as *const _);

    let verts: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    let vao = shaders::Vao::create();
    vao.bind();

    let vbo = shaders::BufferObject::create(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();

    vbo.save_f32_data(&verts);

    let pos_attr = shaders::VertexAttr::create(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
        ptr::null(),
    );

    pos_attr.enable();

    let mut painter = egui_backend::Painter::new(&mut ewindow.window);
    let egui_ctx = egui::Context::default();

    let (width, height) = ewindow.window.get_framebuffer_size();
    let native_pixels_per_point = ewindow.window.get_content_scale().0;

    let mut egui_input_state = egui_backend::EguiInputState::new(egui::RawInput {
        screen_rect: Some(Rect::from_min_size(
            Pos2::new(0f32, 0f32),
            vec2(width as f32, height as f32) / native_pixels_per_point,
        )),
        pixels_per_point: Some(native_pixels_per_point),
        ..Default::default()
    });

    let mut quit = false;

    let mut dock = Dock::default();

    let images = ImageLoader::load_from_dir("./images", &egui_ctx);

    while !ewindow.should_close() {
        // egui_input_state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_frame(egui_input_state.input.take());
        // egui_input_state.input.pixels_per_point = Some(native_pixels_per_point);

        unsafe {
            gl::ClearColor(0.455, 0.302, 0.663, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3)
        }

        egui::TopBottomPanel::top("Top").show(&egui_ctx, |ui| {
            ui.menu_button("File", |ui| {
                {
                    let btn = ui.button("build");
                    if btn.clicked() {
                        project.build();
                    }
                }
                ui.separator();
                {
                    let btn = ui.button("test logs");
                    if btn.clicked() {}
                }
                ui.separator();
                {
                    let btn = ui.button("exit");
                    if btn.clicked() {
                        quit = true;
                    }
                }
            });
        });

        DockArea::new(&mut dock.tree)
            .style(Style::from_egui(egui_ctx.style().as_ref()))
            .show(&egui_ctx, &mut dock.ctx);

        egui::Window::new("Image").show(&egui_ctx, |ui| {
            let folder_img = images.get_texture("graph.png");
            ui.image(&folder_img, folder_img.size_vec2())
        });

        // egui::Window::new("Veiwport").show(&egui_ctx, |_| {});

        // egui::Window::new("Console").show(&egui_ctx, |ui| {
        //     logger::logger_ui(ui, &mut egui_input_state);
        // });

        let egui::FullOutput {
            platform_output,
            repaint_after: _,
            textures_delta,
            shapes,
        } = egui_ctx.end_frame();

        if !platform_output.copied_text.is_empty() {
            egui_backend::copy_to_clipboard(&mut egui_input_state, platform_output.copied_text);
        }

        let clipped_shapes = egui_ctx.tessellate(shapes);
        painter.paint_and_update_textures(1.0, &clipped_shapes, &textures_delta);

        ewindow.update(&mut egui_input_state);

        if quit {
            break;
        }
    }
}
