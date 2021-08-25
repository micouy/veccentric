use veccentric::Fecc;

mod engine;

use engine::{Buffer, Color};

const MAX_FORCE: f64 = 10.0;
const MAX_VELOCITY: f64 = 10.0;
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
}

fn main() -> Result<(), pixels::Error> {
    // Set up state.
    let state = State {
        a: Vehicle::new(50.0, 50.0),
        b: Vehicle::new(10.0, 10.0),
    };

    // Draw state.
    let draw = |State { ref a, ref b }: &State, buffer: &mut Buffer| {
        let (x, y) = a.position.floor().into();
        buffer.put_pixel(x, y, Color::red());

        let (x, y) = b.position.floor().into();
        buffer.put_pixel(x, y, Color::blue());
    };

    // Update state.
    let update = move |State { ref mut a, ref mut b }: &mut State, dt: f64| {
        // Seek.
        let desired_velocity =
            (b.position - a.position).normalize() * MAX_VELOCITY;
        let force = (desired_velocity - a.velocity) / dt;
        a.step(force, dt);

        // Reset a when it reaches b.
        if a.position.dist(b.position) < 2.0 {
        }
    };

    // Run the main loop.
    engine::run(state, update, draw)
}
