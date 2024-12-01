use minifb::{Window, WindowOptions};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use std::time::{Duration, Instant};

fn main() {
    const WIDTH: usize = 300;
    const HEIGHT: usize = 300;
    const FPS: u64 = 60;
    let frame_duration = Duration::from_secs_f64(1.0 / FPS as f64);

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // 创建一个minifb窗口
    let mut window = Window::new(
        "Overlay",
        WIDTH,
        HEIGHT,
        WindowOptions {
            borderless: true,
            ..WindowOptions::default()
        },
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // 使用Windows API使窗口透明和事件穿透
    unsafe {
        let hwnd = HWND::from_raw(window.get_window_handle() as *mut _);
        let styles = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
        let new_styles = WINDOW_EX_STYLE(styles)
            | WS_EX_LAYERED
            | WS_EX_TRANSPARENT
            | WS_EX_TOPMOST;
        SetWindowLongW(hwnd, GWL_EXSTYLE, new_styles.0 as i32);
        SetLayeredWindowAttributes(hwnd, COLORREF(0), 255, LWA_ALPHA);
    }

    // 主循环
    while window.is_open() {
        let start_time = Instant::now();

        // 绘制正方形
        for y in 100..200 {
            for x in 100..200 {
                buffer[y * WIDTH + x] = 0x00FF00; // 绿色
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        // 计算帧时间，并确保每帧间隔约16.67毫秒
        let elapsed_time = start_time.elapsed();
        if elapsed_time < frame_duration {
            std::thread::sleep(frame_duration - elapsed_time);
        }
    }
}
