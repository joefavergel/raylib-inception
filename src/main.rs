use raylib_ffi::{
    BeginDrawing,
    CloseWindow,
    DrawPixel,
    DrawTexture,
    EndDrawing,
    InitWindow,
    LoadTextureFromImage,
    WindowShouldClose,
    Color,
    colors::WHITE,
    enums,
    Image,
    Texture2D, UpdateTexture
};
use std::ffi::c_void;

fn main() {
    init_gui();

    let mut screen_buffer_data = [Color{r:0, b:0, g:0, a:0} ; 400*300];
    let screen_buffer = Image {
        data: screen_buffer_data.as_mut_ptr() as *mut c_void,
        width: 400,
        height: 300,
        mipmaps: 1,
        format: enums::PixelFormat::R8g8b8a8 as i32
    };
    let screen_buffer_texture = load_texture_from_image(screen_buffer);

    while ! window_should_close() {
        begin_drawing();
        draw_next_frame(&mut screen_buffer_data);
        update_texture(screen_buffer_texture, &screen_buffer_data);
        // draw_pixel(200, 200, Color{r:0xFF, g:0, b: 0, a:0xFF});
        draw_texture(screen_buffer_texture, 0, 0, WHITE);
        end_drawing()
    }

    close_window();
}

fn init_gui() {
    unsafe {
        InitWindow(400, 400, "Fire".as_ptr() as *const i8)
    }
}

fn window_should_close() -> bool {
    unsafe { WindowShouldClose() }
}

fn begin_drawing() {
    unsafe {
        BeginDrawing()
    }
}

fn end_drawing() {
    unsafe {
        EndDrawing()
    }
}

fn close_window() {
    unsafe {
        CloseWindow()
    }
}

fn draw_pixel(x: i32, y: i32, color: Color) {
    unsafe {
        DrawPixel(x, y, color)
    }
}

fn load_texture_from_image(img: Image) -> Texture2D {
    unsafe {
        LoadTextureFromImage(img)
    }
}

fn draw_texture(tex: Texture2D, x: i32, y: i32, tint: Color) {
    unsafe {
        DrawTexture(tex, x, y, tint)
    }
}

fn update_texture(tex: Texture2D, data: &[Color]) {
    unsafe {
        UpdateTexture(tex, data.as_ptr() as *const c_void);
    }
}

fn draw_next_frame(screen:  &mut [Color]){
    let pixel = &mut screen[200*400 + 200];
    *pixel = Color{r:0xFF,g:0,b:0,a:0xFF};

}