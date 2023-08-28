use raylib_ffi::{
    InitWindow,
    CloseWindow,
    WindowShouldClose,
    ClearBackground,
    BeginDrawing,
    EndDrawing,
    colors,
    SetTargetFPS

};

pub fn init_window(width: i32, height: i32, title: *const i8) {
    unsafe {
        InitWindow(width, height, title)
    }
}

pub fn close_window() {
    unsafe { CloseWindow() }
}

pub fn window_should_close() -> bool{
    unsafe { WindowShouldClose() }
}

pub fn begin_drawing() {
    unsafe { BeginDrawing()}
}

pub fn end_drawing() {
    unsafe { EndDrawing() }
}

pub fn clear_background() {
    unsafe {
        ClearBackground(colors::BLACK)
    }
}

pub fn set_target_fps(fps: i32) {
    unsafe { SetTargetFPS(fps) }
}
