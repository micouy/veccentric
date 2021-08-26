use veccentric::Fecc;

use std::time::Instant;

mod engine;

use engine::{Buffer, Color};

const MAX_FORCE: f64 = 100.0;
const MAX_VELOCITY: f64 = 20.0;
const MASS: f64 = 1.0;

struct Vehicle {
    position: Fecc,
    velocity: Fecc,
}

impl Vehicle {
	fn new(x: f64, y: f64) -> Self {
		Self {
			position: (x, y).into(),
			velocity: Fecc::zero(),
		}
	}

    fn step(&mut self, force: Fecc, dt: f64) {
        let force = force.limit(MAX_FORCE);
        let acceleration = force / MASS;
        self.velocity = (self.velocity + acceleration * dt).limit(MAX_VELOCITY);
        self.position = self.position + self.velocity * dt;
    }
}

struct State {
    a: Vehicle,
    b: Vehicle,
	start: Instant,
}

fn main() -> Result<(), pixels::Error> {
    // Set up state.
    let state = State {
        a: Vehicle::new(50.0, 50.0),
        b: Vehicle::new(10.0, 10.0),
		start: Instant::now(),
    };
	let background = Color::black();

    // Draw state.
    let draw = |State { ref a, ref b, .. }: &State, buffer: &mut Buffer| {
        buffer.draw_point(a.position, Color::white());
        buffer.draw_point(b.position, Color::white());
    };

    // Update state.
    let update = move |State { ref mut a, ref mut b, ref mut start }: &mut State, dt: f64| {
        // Seek.
        let desired_velocity =
            (b.position - a.position).normalize() * MAX_VELOCITY;
        let force = (desired_velocity - a.velocity) / dt;
        a.step(force, dt);

        // Reset a when it reaches b.
        if start.elapsed().as_secs_f64() >= 8.0 {
			a.position = (50.0, 50.0).into();
			a.velocity = Fecc::zero();
			*start = Instant::now();
        }
    };

    // Run the main loop.
    engine::run(state, update, draw, background)
}
