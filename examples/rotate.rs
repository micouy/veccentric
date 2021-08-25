use std::time::{Instant};

use veccentric::Fecc;

mod engine;

use engine::{Buffer, Color, WIDTH, HEIGHT};

struct State {
	point: Fecc,
	start: Instant,
}

fn main() -> Result<(), pixels::Error> {
    // Set up state.
	let original = (25.0, 0.0).into();
	let center = Fecc::new(WIDTH as f64 / 2.0, HEIGHT as f64 / 2.0);
    let state = State {
		point: original,
		start: Instant::now(),
	};

    // Draw state.
    let draw = move |State { ref point, .. }: &State, buffer: &mut Buffer| {
        let (x, y) = (point + center).floor().into();
        buffer.put_pixel(x, y, Color::red());
    };

    // Update state.
    let update = move |State { ref mut point, ref mut start }: &mut State, dt: f64| {
		*point = original.rotate(start.elapsed().as_secs_f64());
    };

    // Run the main loop.
    engine::run(state, update, draw)
}
