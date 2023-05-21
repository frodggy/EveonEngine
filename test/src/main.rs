use egui_glfw_gl as egui_backend;
use eveon_engine::core::graphics::window::*;
use std::time::Instant;

use egui_backend::egui::{vec2, Color32, Image, Pos2, Rect};
use egui_glfw_gl::egui;
use egui_glfw_gl::glfw::Context;

const PIC_WIDTH: i32 = 320;
const PIC_HEIGHT: i32 = 192;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));

    let mut ewindow = EvWindow::new("My Game", 1080, 720);

    ewindow.window.set_char_polling(true);
    ewindow.window.set_cursor_pos_polling(true);
    ewindow.window.set_key_polling(true);
    ewindow.window.set_mouse_button_polling(true);

    gl::load_with(|symbol| ewindow.window.get_proc_address(symbol) as *const _);

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

    let start_time = Instant::now();
    let srgba = vec![Color32::BLACK; (PIC_HEIGHT * PIC_WIDTH) as usize];

    let plot_tex_id = painter.new_user_texture(
        (PIC_WIDTH as usize, PIC_HEIGHT as usize),
        &srgba,
        egui::TextureFilter::Linear,
    );

    let mut sine_shift = 0f32;
    let mut amplitude = 50f32;
    let mut test_str =
        "A text box to write in. Cut, copy, paste commands are available.".to_owned();

    let mut quit = false;

    while !ewindow.should_close() {
        egui_input_state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_frame(egui_input_state.input.take());
        egui_input_state.input.pixels_per_point = Some(native_pixels_per_point);

        unsafe {
            gl::ClearColor(0.455, 0.302, 0.663, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let mut srgba: Vec<Color32> = Vec::new();
        let mut angle = 0f32;

        for y in 0..PIC_HEIGHT {
            for x in 0..PIC_WIDTH {
                srgba.push(Color32::BLACK);
                if y == PIC_HEIGHT - 1 {
                    let y = amplitude * (angle * std::f32::consts::PI / 180f32 + sine_shift).sin();
                    let y = PIC_HEIGHT as f32 / 2f32 - y;
                    srgba[(y as i32 * PIC_WIDTH + x) as usize] = Color32::YELLOW;
                    angle += 360f32 / PIC_WIDTH as f32;
                }
            }
        }
        sine_shift += 0.1f32;

        //This updates the previously initialized texture with new data.
        //If we weren't updating the texture, this call wouldn't be required.
        painter.update_user_texture_data(&plot_tex_id, &srgba);

        egui::TopBottomPanel::top("Top").show(&egui_ctx, |ui| {
            ui.menu_button("File", |ui| {
                {
                        let _ = ui.button("test 1");
                }
                ui.separator();
                {
                    let _ = ui.button("test 2");
                }
            });
        });


        let egui::FullOutput {
            platform_output,
            repaint_after: _,
            textures_delta,
            shapes,
        } = egui_ctx.end_frame();

        //Handle cut, copy text from egui
        if !platform_output.copied_text.is_empty() {
            egui_backend::copy_to_clipboard(&mut egui_input_state, platform_output.copied_text);
        }

        //Note: passing a bg_color to paint_jobs will clear any previously drawn stuff.
        //Use this only if egui is being used for all drawing and you aren't mixing your own Open GL
        //drawing calls with it.
        //Since we are custom drawing an OpenGL Triangle we don't need egui to clear the background.

        let clipped_shapes = egui_ctx.tessellate(shapes);
        painter.paint_and_update_textures(1.0, &clipped_shapes, &textures_delta);

        ewindow.update(&mut egui_input_state);

        if quit {
            break;
        }
    }
}
