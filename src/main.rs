mod pendulum;

use nannou::prelude::*;
use pendulum::{state, Pendulum};

struct Model {
    pendulum: Pendulum,
    time: f32,
}

fn main() {
    nannou::app(model).simple_window(view).event(event).run()
}

fn model(app: &App) -> Model {
    Model {
        pendulum: state::new(),
        time: app.time,
    }
}

fn event(app: &App, model: &mut Model, _event: Event) {
    //time passed since last update
    let delta_t = app.time - model.time;

    //update physics
    model.pendulum.update(delta_t);

    // update the time in the model
    model.time = app.time;
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    // Clear the background to blue.
    draw.background().color(CORNFLOWERBLUE);

    // The line of the pendulum
    draw.line()
        .start(vec2(model.pendulum.root.0, model.pendulum.root.1))
        .end(vec2(model.pendulum.position.0, model.pendulum.position.1))
        .color(BLACK)
        .start_cap_round()
        .weight(5.0);

    // The weight of the pendulum
    draw.ellipse()
        .x_y(model.pendulum.position.0, model.pendulum.position.1)
        .w_h(100.0, 100.0)
        .color(BLACK);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
