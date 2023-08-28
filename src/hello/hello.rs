use raylib_ffi::{
    InitWindow,
    CloseWindow,
    WindowShouldClose,
    ClearBackground,
    BeginDrawing,
    EndDrawing,
    colors,
    Color,
    DrawText
};


const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;

pub fn say_hello() {
    init_window();

    while ! window_should_close() {
        begin_drawing();
        clear_background();
        draw_text();
        end_drawing();
    }

    close_window();
}

fn init_window() {
    unsafe {
        InitWindow(WIDTH, HEIGHT, "Hello, World!".as_ptr() as *const i8)
    }

}

fn close_window() {
    unsafe { CloseWindow() }
}

fn window_should_close() -> bool{
    unsafe { WindowShouldClose() }
}

fn begin_drawing() {
    unsafe { BeginDrawing()}
}

fn end_drawing() {
    unsafe { EndDrawing() }
}

fn clear_background() {
    unsafe {
        ClearBackground(colors::BLACK)
    }
}

fn draw_text() {
    unsafe {
        DrawText(
            "Hello, World!".as_ptr() as *const i8,
            200,
            220,
            36,
            Color{r:44,g:217,b:17,a:0xFF}
        )
    }
}
