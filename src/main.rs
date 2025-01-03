use egui::{epaint::Shadow};
use macroquad::prelude::*;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    loop {
        clear_background(DARKGRAY);

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


        draw_circle(screen_width()/2.0, screen_height()/2.0, 16.0, WHITE);

        
        egui_macroquad::draw();
        next_frame().await;
    }
}