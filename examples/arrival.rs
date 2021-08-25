use veccentric::Fecc;

mod pixels_helper;

use pixels_helper::{run, Buffer, Color, HEIGHT, WIDTH};

struct State {
    pos_a: Fecc,
    vel_a: Fecc,

    pos_b: Fecc,
    vel_b: Fecc,
}

fn main() -> Result<(), pixels::Error> {
    let state = State {
        pos_a: (10.0, 50.0).into(),
        vel_a: Fecc::zero(),
        pos_b: (25.0, 25.0).into(),
        vel_b: Fecc::zero(),
    };

    let window_size: Fecc = (WIDTH as f64, HEIGHT as f64).into();

    let draw = |state: &State, buffer: &mut Buffer| {
        let (x, y) = state.pos_a.floor().into();
        buffer.put_pixel(x, y, Color::red());

        let (x, y) = state.pos_b.floor().into();
        buffer.put_pixel(x, y, Color::blue());
    };

    let update = move |state: &mut State, dt: f64| {
        let max_speed = 1.0;
        let max_force = 400.0;
        let mass = 1.0;
        let slowing_distance = 50.0;

        let offset = state.pos_b - state.pos_a;
        let distance = offset.mag();
        let ramped_speed = max_speed * (distance / slowing_distance);
        let clipped_speed = ramped_speed.min(max_speed);
        let desired_velocity = offset * (clipped_speed / distance);
        let steering = desired_velocity - state.vel_a;

        let steering_force = steering.limit(max_force);
        let acceleration = steering_force / mass;
        state.vel_a = (state.vel_a + acceleration * dt).limit(max_speed);
        state.pos_a = state.pos_a + state.vel_a;

        // state.pos_a = state.pos_a % window_size;
        // state.pos_b = state.pos_b % window_size;
    };

    run(state, update, draw)
}
