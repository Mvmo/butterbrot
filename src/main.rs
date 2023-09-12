use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Butterbrot",
        WIDTH,
        HEIGHT,
        WindowOptions::default()
    ).unwrap();

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut count = 0;
        for i in buffer.iter_mut() {
            count += 1;

            if count >= 50000 {
                *i = 0xAA;
            } else {
                *i = 0xFF;
            }

        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
