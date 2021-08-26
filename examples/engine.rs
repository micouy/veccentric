#![allow(dead_code)]
use veccentric::Fecc;

use std::time::Instant;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 64;
const SCALE: f64 = 5.0;

#[derive(Copy, Clone)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn red() -> Self {
        Color(0xff, 0x00, 0x00)
    }

    pub fn green() -> Self {
        Color(0x00, 0xff, 0x00)
    }

    pub fn blue() -> Self {
        Color(0x00, 0x00, 0xff)
    }

    pub fn white() -> Self {
        Color(0xff, 0xff, 0xff)
    }

    pub fn black() -> Self {
        Color(0x00, 0x00, 0x00)
    }
}

pub fn run<S, U, D>(
    mut state: S,
    mut update: U,
    mut draw: D,
    background: Color,
) -> Result<(), Error>
where
    S: 'static,
    U: FnMut(&mut S, f64) + 'static,
    D: FnMut(&S, &mut Buffer) + 'static,
{
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size =
            LogicalSize::new(WIDTH as f64 * SCALE, HEIGHT as f64 * SCALE);

        WindowBuilder::new()
            .with_title(get_exec_name())
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, &window);

        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut dt = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            let mut buffer = Buffer::new(pixels.get_frame());
            update(&mut state, dt.elapsed().as_secs_f64());
            buffer.clear(background);
            draw(&state, &mut buffer);
            dt = Instant::now();

            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;

                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;

                return;
            }

            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            window.request_redraw();
        }
    });
}

pub struct Buffer<'a> {
    pixels: &'a mut [u8],
}

impl<'a> Buffer<'a> {
    fn new(pixels: &'a mut [u8]) -> Self {
        Self { pixels }
    }

    #[allow(clippy::many_single_char_names)]
    pub fn draw_point(&mut self, position: Fecc, Color(r, g, b): Color) {
        let (x, y) = position.floor().into();

        if let Some(ix) = Self::ix(x, y) {
            self.pixels[ix..(ix + 4)].copy_from_slice(&[r, g, b, 0xff]);
        }
    }

    fn clear(&mut self, Color(r, g, b): Color) {
        for pixel in self.pixels.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[r, g, b, 0xff]);
        }
    }

    fn ix(x: i64, y: i64) -> Option<usize> {
        if (0..WIDTH as i64).contains(&x) && (0..HEIGHT as i64).contains(&y) {
            Some(((x + y * WIDTH as i64) * 4) as usize)
        } else {
            None
        }
    }
}

fn get_exec_name() -> String {
    std::env::current_exe()
        .ok()
        .and_then(|pb| pb.file_name().map(|s| s.to_os_string()))
        .and_then(|s| s.into_string().ok())
        .unwrap_or("veccentric".to_string())
}
