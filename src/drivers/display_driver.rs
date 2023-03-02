use minifb;

#[derive(Clone, Copy)]
enum Pixel {
    ON,
    OFF,
}

pub struct Display {
    scale: usize,
    width: usize,
    height: usize,
    buffer: Vec<Pixel>,
    window: minifb::Window,
}

impl Display {
    const COLS: usize = 64;
    const ROWS: usize = 32;

    pub fn new(title: &str, scale: usize) -> Self {
        let (width, height) = (Self::COLS * scale, Self::ROWS * scale);
        let buffer = vec![Pixel::OFF; Self::COLS * Self::ROWS];

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
        if x > Self::COLS {
            x = 0;
        }

        if y > Self::ROWS {
            y = 0;
        }

        let pixel_loc = x + (y * Self::COLS);

        self.buffer[pixel_loc] = match self.buffer[pixel_loc] {
            Pixel::ON => Pixel::OFF,
            Pixel::OFF => Pixel::ON,
        };

        match self.buffer[pixel_loc] {
            Pixel::ON => false,
            Pixel::OFF => true,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.iter_mut().for_each(|p| *p = Pixel::OFF);
    }

    pub fn render(&mut self) {
        let mut buffer: Vec<u32> = vec![0; self.width * self.height];

        for (index, pixel) in self.buffer.iter_mut().enumerate() {
            let x = (index % Self::COLS) * self.scale;
            let y = ((index / Self::COLS) as f32).floor() as usize * self.scale;

            match pixel {
                Pixel::ON => {
                    for row in y..(y + self.scale) {
                        for col in x..(x + self.scale) {
                            let index = row * self.width + col;
                            if index < buffer.len() {
                                buffer[index] = 0x00ff00;
                            }
                        }
                    }
                }
                Pixel::OFF => {}
            }
        }

        self.window
            .update_with_buffer(&buffer, self.width, self.height)
            .unwrap_or_else(|e| panic!("{}", e));
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn is_key_down(&self, key: minifb::Key) -> bool {
        self.window.is_key_down(key)
    }
}
