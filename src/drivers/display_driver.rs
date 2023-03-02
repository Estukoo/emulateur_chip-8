use minifb;

pub struct Display {
    scale: usize,
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    window: minifb::Window,
}

impl Display {
    const COLS: usize = 64;
    const ROWS: usize = 32;

    pub fn new(title: &str, scale: usize) -> Self {
        let (width, height) = (Self::COLS * scale, Self::ROWS * scale);
        let buffer = vec![0; width * height];

        let mut window =
            minifb::Window::new(title, width, height, minifb::WindowOptions::default())
                .unwrap_or_else(|e| panic!("{}", e));

        window.limit_update_rate(Some(std::time::Duration::from_micros(16_600)));

        Display {
            scale: scale,
            width: width,
            height: height,
            buffer: buffer,
            window: window,
        }
    }

    pub fn _get_dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn set_pixel(&mut self, mut x: usize, mut y: usize) -> bool {
        x *= self.scale;
        y *= self.scale;

        for row in y..(y + self.scale) {
            for col in x..(x + self.scale) {
                let index = row * self.width + col;
                if index < self.buffer.len() {
                    self.buffer[index] = 0x00ff00;
                }
            }
        }

        true
    }

    pub fn test_render(&mut self) {
        self.set_pixel(0, 0);
        self.set_pixel(5, 2);
    }

    pub fn clear(&mut self) {
        self.buffer.iter_mut().for_each(|p| *p = 0);
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap_or_else(|e| panic!("{}", e));
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn is_key_down(&self, key: minifb::Key) -> bool {
        self.window.is_key_down(key)
    }
}
