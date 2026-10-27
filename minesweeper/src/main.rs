use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    loop {
        clear_background(BLACK);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, WHITE);
        next_frame().await
    }
}
