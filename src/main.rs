mod drivers {
    pub mod display_driver;
}

use drivers::display_driver::Display;

fn main() {
    let mut wnd = Display::new("Chip-8 Emulator - Alpha", None);

    println!(
        "Dims : {} : {}",
        wnd.get_dimensions().0,
        wnd.get_dimensions().1
    );

    let mut x: usize = 0;
    let mut y: usize = 0;

    while wnd.is_open() && !wnd.is_key_down(minifb::Key::Escape) {
        wnd.update();
        wnd.set_pixel(x, y, (255, 255, 0));
        x += 1;
        y += 1;
    }
}
