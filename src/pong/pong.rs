use raylib_ffi::{Color, colors, DrawText, TextFormat};

use crate::core::*;

const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;

pub fn start() {
    let title = "Hello, Pong!";
    let description = "A blad pong game";

    init_window(WIDTH, HEIGHT, title.as_ptr() as *const i8);
    set_target_fps(60);

    while ! window_should_close() {
        begin_drawing();
        clear_background();
        draw_wellcome(title, description);
        end_drawing();
    }
}

fn draw_wellcome(title: &str, description: &str) {
    let intit_score = 000000;
    let init_lives = 05;

    unsafe {
        DrawText(
            TextFormat("Score: %08i".as_ptr() as *const i8, intit_score as *const i32), 
            460, 
            10, 
            20, 
            colors::RED
        );
    }

    unsafe {
        DrawText(
            TextFormat("Lives: %02i".as_ptr() as *const i8, init_lives as *const i32),
            10, 
            10,
            20,
            colors::BLUE
        )
    }

    unsafe {
        DrawText(
            title.as_ptr() as *const i8,
            120,
            120,
            36,
            Color{r:44,g:217,b:17,a:0xFF}
        )
    }
    unsafe {
        DrawText(
            description.as_ptr() as *const i8,
            240,
            160,
            22,
            Color{r:44,g:217,b:17,a:0xFF}
        )
    }
}
