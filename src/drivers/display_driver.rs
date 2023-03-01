use minifb;

pub struct Display {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    window: minifb::Window,
}

impl Display {
    const DEFAULT_WIDTH: usize = 640;
    const DEFAULT_HEIGHT: usize = 360;

    // const COLS: usize = 64;
    // const ROWS: usize = 32;

    pub fn new(title: &str, dimensions: Option<(usize, usize)>) -> Self {
        let (width, height) = dimensions.unwrap_or((Self::DEFAULT_WIDTH, Self::DEFAULT_HEIGHT));
        let buffer = vec![0; width * height];

        let mut window =
            minifb::Window::new(title, width, height, minifb::WindowOptions::default())
                .unwrap_or_else(|e| panic!("{}", e));

        window.limit_update_rate(Some(std::time::Duration::from_micros(16_600)));

        Display {
            width,
            height,
            buffer,
            window,
        }
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: (u32, u32, u32)) {
        let index = y * self.width + x;
        if index < self.buffer.len() {
            self.buffer[index] = ((color.0 as u32) << 24)
                | ((color.1 as u32) << 16)
                | ((color.2 as u32) << 8)
                | 0xFF;
        }
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
