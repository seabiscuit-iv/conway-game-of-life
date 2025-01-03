use egui::{epaint::Shadow, Align2};
use macroquad::{prelude::*, window};
use miniquad::{log, window::screen_size};


const BLOCK_SIZE : f32 = 15.0;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let mut view_pos = vec2(0.0, 0.0);

    let mut pixels = Vec::<Vec2>::new();
    let mut ui_clicked = false;

    loop {
        clear_background(DARKGRAY);

        let mut step = false;

        egui_macroquad::ui(|egui_ctx| {
            let window = egui::Window::new("Controls")
                .frame(egui::Frame::window(&egui::Style::default()).shadow(Shadow::NONE).inner_margin(12.0))
                .resizable(false)
                .title_bar(false)
                .anchor(Align2::RIGHT_BOTTOM, egui::Vec2::new(-10.0, -10.0))
                // .fixed_rect(egui::Rect::from_center_size(egui::pos2(100.0, 100.0), egui::vec2(100.0, 40.0)))
                .show(egui_ctx, |ui| {
                    ui.label("This is a test");
                    ui.label("This is a test");
                    ui.label("This is another Test");
                    let button = ui.button("Step");
                    step = button.clicked();
                    ui_clicked = button.hovered();
                });

            ui_clicked = ui_clicked || window.unwrap().response.hovered();
        });


        
        if !ui_clicked {
            if is_mouse_button_down(MouseButton::Left) && (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)){
                view_pos += vec2(screen_width()/2.0, screen_height()/2.0)*mouse_delta_position();
            } else {
                mouse_delta_position();
                if is_mouse_button_pressed(MouseButton::Left) {
                    let clickpos = vec2(mouse_position().0, mouse_position().1) + view_pos;
                    let clicked_block = (clickpos / BLOCK_SIZE).floor();

                    let clicked_block_index = pixels.iter().enumerate().find(|x| {
                        *(x.1) == clicked_block
                    });

                    // println!("{}", clicked_block)
                    match clicked_block_index {
                        Some((i, _)) => {
                            pixels.remove(i);
                        },
                        None =>  {
                            pixels.push(clicked_block);
                        }
                    }
                }
            }
        }



        
        let block_width = (screen_width() / BLOCK_SIZE).ceil() as u32;
        let block_height = (screen_height() / BLOCK_SIZE).ceil() as u32;
        
        //cleanup
        pixels = pixels.iter().filter(|block| {block.x >= 0.0 && block.y >= 0.0 && block.x < block_width as f32 && block.y < block_height as f32}).map(|x| *x).collect();

        // conway's game of life
        if step {
            pixels = pixels.iter().map(|vec| *vec + vec2(-1.0, 0.0)).collect();
        }



        // draw_circle(screen_width()/2.0 - view_pos.x, screen_height()/2.0 - view_pos.y, 16.0, WHITE);
        // draw_circle(-view_pos.x, -view_pos.y, 16.0, WHITE);

        for x in 0..block_width {
            for y in 0..block_height {

                if pixels.contains(&vec2(x as f32, y as f32)) {
                    draw_rectangle(x as f32 *BLOCK_SIZE - view_pos.x,  y as f32 * BLOCK_SIZE - view_pos.y, BLOCK_SIZE, BLOCK_SIZE, LIGHTGRAY);
                } else {
                    draw_rectangle_lines(x as f32 *BLOCK_SIZE - view_pos.x,  y as f32 * BLOCK_SIZE - view_pos.y, BLOCK_SIZE, BLOCK_SIZE, 0.5, GRAY);
                }
                // draw_circle(x as f32 * BLOCK_SIZE, y as f32 * BLOCK_SIZE, 1.0, WHITE);
            }
        }

        
        egui_macroquad::draw();
        next_frame().await;
    }
}