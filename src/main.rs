use egui::{epaint::Shadow};
use macroquad::prelude::*;
use miniquad::{log, window::screen_size};

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let mut view_pos = vec2(0.0, 0.0);

    loop {
        clear_background(DARKGRAY);

        println!("{:?}", screen_size());

        if is_mouse_button_down(MouseButton::Left) {
            view_pos += vec2(screen_width()/2.0, screen_height()/2.0)*mouse_delta_position();
        } else {
            mouse_delta_position();
        }

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Controls")
                .frame(egui::Frame::window(&egui::Style::default()).shadow(Shadow::NONE).inner_margin(12.0))
                .resizable(false)
                .title_bar(false)
                // .fixed_rect(egui::Rect::from_center_size(egui::pos2(100.0, 100.0), egui::vec2(100.0, 40.0)))
                .show(egui_ctx, |ui| {
                    ui.label("This is a test");
                    ui.label("This is a test");
                    ui.label("This is another Test");
                    ui.button("Cookie")
                });
        });


        // draw_circle(screen_width()/2.0 - view_pos.x, screen_height()/2.0 - view_pos.y, 16.0, WHITE);
        draw_circle(-view_pos.x, -view_pos.y, 16.0, WHITE);

        
        egui_macroquad::draw();
        next_frame().await;
    }
}