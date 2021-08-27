use veccentric::Fecc;

mod engine;

use engine::{Buffer, Color};

// Play with the params.
const MAX_FORCE: f64 = 100.0;
const MAX_VELOCITY: f64 = 1_000_000.0;
const G: f64 = 100.0;

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
    a: Vehicle,
    b: Vehicle,
    c: Vehicle,
}

fn main() -> Result<(), pixels::Error> {
    // Set up state.
    let state = State {
        a: Vehicle::new(16.0, 16.0, 10.0),
        b: Vehicle::new(48.0, 16.0, 15.0),
        c: Vehicle::new(32.0, 48.0, 20.0),
    };
    let background = Color(0x00, 0x00, 0x11);

    // Draw state.
    let draw = |State {
                    ref a,
                    ref b,
                    ref c,
                }: &State,
                buffer: &mut Buffer| {
        buffer.draw_point(a.position, Color(0xff, 0xff, 0x77));
        buffer.draw_point(b.position, Color(0xff, 0xff, 0x77));
        buffer.draw_point(c.position, Color(0xff, 0xff, 0x77));
    };

    let calc_force = |body: &Vehicle, other_body: &Vehicle| {
        let radius = other_body.position - body.position;

        // F = r_hat * G * M * m / |r|^2
        radius.normalize() * G * body.mass * other_body.mass
            / radius.mag_squared()
    };

    // Update state.
    let update = move |State {
                           ref mut a,
                           ref mut b,
                           ref mut c,
                       }: &mut State,
                       dt: f64| {
        let a_force = calc_force(a, b) + calc_force(a, c);
        let b_force = calc_force(b, a) + calc_force(b, c);
        let c_force = calc_force(c, a) + calc_force(c, b);

        a.step(a_force, dt);
        b.step(b_force, dt);
        c.step(c_force, dt);
    };

    // Run the main loop.
    engine::run(state, update, draw, background)
}
