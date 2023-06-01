#![no_std]
#![no_main]

use core::mem;

use uefi::{
    prelude::*,
    proto::{console::gop::GraphicsOutput, rng::Rng},
    Result,
};

mod buffer;

use buffer::Buffer;

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Get a random `usize` value.
fn get_random_usize(rng: &mut Rng) -> usize {
    let mut buf = [0; mem::size_of::<usize>()];
    rng.get_rng(None, &mut buf).expect("get_rng failed");
    usize::from_le_bytes(buf)
}

fn draw_sierpinski(bt: &BootServices) -> Result {
    // Open graphic output protocol
    let gop_handle = bt.get_handle_for_protocol::<GraphicsOutput>()?;
    let mut gop = bt.open_protocol_exclusive::<GraphicsOutput>(gop_handle)?;

    // Open random number generator protocol.
    let rng_handle = bt.get_handle_for_protocol::<Rng>()?;
    let mut rng = bt.open_protocol_exclusive::<Rng>(rng_handle)?;

    // Create a buffer to draw into.
    let (width, height) = gop.current_mode_info().resolution();
    let mut buffer = Buffer::new(width, height);

    // Initialize the buffer with a simple gradient background.
    for y in 0..height {
        let r = ((y as f32) / ((height - 1) as f32)) * 255.0;
        for x in 0..width {
            let g = ((x as f32) / ((width - 1) as f32)) * 255.0;
            let pixel = buffer.pixel(x, y).unwrap();
            pixel.red = r as u8;
            pixel.green = g as u8;
            pixel.blue = 255;
        }
    }

    let size = Point::new(width as f32, height as f32);

    // Define the vertices of a big triangle.
    let border = 20.0;
    let triangle = [
        Point::new(size.x / 2.0, border),
        Point::new(border, size.y - border),
        Point::new(size.x - border, size.y - border),
    ];

    // `p` is the point to draw. Start at the center of the triangle.
    let mut p = Point::new(size.x / 2.0, size.y / 2.0);

    // Loop forever, drawing the frame after each new point is changed.
    loop {
        // Choose one of the triangle's vertices at random.
        let v = triangle[get_random_usize(&mut rng) % 3];

        // Move `p` halfway to the chosen vertex.
        p.x = (p.x + v.x) * 0.5;
        p.y = (p.y + v.y) * 0.5;

        // Set `p` to black.
        let pixel = buffer.pixel(p.x as usize, p.y as usize).unwrap();
        pixel.red = 0;
        pixel.green = 100;
        pixel.blue = 0;

        // Draw the buffer to the screen.
        buffer.blit(&mut gop)?;
    }
}

#[entry]
fn main(_handle: Handle, mut st: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut st).unwrap();
    let bt = st.boot_services();
    draw_sierpinski(bt).unwrap();
    Status::SUCCESS
}
