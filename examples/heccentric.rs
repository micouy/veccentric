use veccentric::Fecc;

mod engine;

use engine::{Buffer, Color};

const MAX_FORCE: f64 = 1000_000.0;
const MAX_VELOCITY: f64 = 1000_000.0;
const SUN_MASS: f64 = 10_000.0;
const EARTH_MASS: f64 = 0.1;
const G: f64 = 1.0;

struct Vehicle {
    position: Fecc,
    velocity: Fecc,
    mass: f64,
}

impl Vehicle {
    fn new(x: f64, y: f64, mass: f64) -> Self {
        Self {
            position: (x, y).into(),
            velocity: Fecc::zero(),
            mass,
        }
    }

    fn step(&mut self, force: Fecc, dt: f64) {
        let force = force.limit(MAX_FORCE);
        let acceleration = force / self.mass;
        self.velocity = (self.velocity + acceleration * dt).limit(MAX_VELOCITY);
        self.position = self.position + self.velocity * dt;
    }
}

struct State {
    sun: Vehicle,
    earth: Vehicle,
}

fn main() -> Result<(), pixels::Error> {
    // Set up state.
    let state = State {
        sun: Vehicle::new(32.0, 32.0, SUN_MASS),
        earth: Vehicle {
            position: (15.0, 15.0).into(),
            velocity: (-10.0, 10.0).into(),
            mass: EARTH_MASS,
        },
    };
    let background = Color(0x00, 0x00, 0x11);

    // Draw state.
    let draw = |State { ref earth, ref sun }: &State, buffer: &mut Buffer| {
        buffer.draw_point(sun.position, Color(0xff, 0xff, 0x00));
        buffer.draw_point(earth.position, Color(0x11, 0x11, 0xff));
    };

    // Update state.
    let update = move |State {
                           ref mut sun,
                           ref mut earth,
                       }: &mut State,
                       dt: f64| {
        let radius = sun.position - earth.position;
        // F = r_hat * G * M * m / |r|^2
        let force = radius.normalize() * G * SUN_MASS * EARTH_MASS
            / radius.mag_squared();
        earth.step(force, dt);
    };

    // Run the main loop.
    engine::run(state, update, draw, background)
}
