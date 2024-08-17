use macroquad::prelude::*;

#[macroquad::main("Jetpack-Quad")]
async fn main() {
    loop {
        clear_background(BLACK);

        next_frame().await
    }
}
