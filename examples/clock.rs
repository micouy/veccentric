use std::{f64::consts::PI, time::Instant};

use veccentric::Fecc;

mod engine;

use engine::{Buffer, Color, HEIGHT, WIDTH};

struct State {
    seconds: Fecc,
    minutes: Fecc,
    hours: Fecc,
    start: Instant,
}

fn main() -> Result<(), pixels::Error> {
    // Set up state.
    let original_s = (0.0, -25.0).into();
    let original_m = (0.0, -22.0).into();
    let original_h = (0.0, -19.0).into();
    let center = Fecc::new(WIDTH as f64 / 2.0, HEIGHT as f64 / 2.0);
    let state = State {
        seconds: original_s,
        minutes: original_m,
        hours: original_h,
        start: Instant::now(),
    };
    let background = Color::white();

    // Draw state.
    let draw = move |State {
                         ref seconds,
                         ref minutes,
                         ref hours,
                         ..
                     }: &State,
                     buffer: &mut Buffer| {
        buffer.draw_point(seconds + center, Color::black());
        buffer.draw_point(minutes + center, Color::black());
        buffer.draw_point(hours + center, Color::black());
    };

    let angle = |start: &Instant, power| {
        (start.elapsed().as_secs_f64() / 60.0_f64.powf(power)).floor()
            * 2.0
            * PI
            / 60.0
    };

    // Update state.
    let update = move |state: &mut State, _dt: f64| {
        state.seconds = original_s.rotate(angle(&state.start, 0.0));
        state.minutes = original_m.rotate(angle(&state.start, 1.0));
        state.hours = original_h.rotate(angle(&state.start, 2.0));
    };

    // Run the main loop.
    engine::run(state, update, draw, background)
}
