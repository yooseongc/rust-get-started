
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    x: f32,
    y: f32,
    x_speed: f32,
    y_speed: f32,
}

fn model(app: &App) -> Model {
    let x = 100.0;
    let y = 100.0;
    let x_speed = 2.5;
    let y_speed = 2.0;

    let _window = app.new_window().size(800, 200).view(view).build().unwrap();
    Model {
        x,
        y,
        x_speed,
        y_speed,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // add the current speed to the position
    model.x = model.x + model.x_speed;
    model.y = model.y + model.y_speed;

    let win_rect = app.window_rect();

    if (model.x > win_rect.right()) || (model.x < win_rect.left()) {
        model.x_speed = model.x_speed * (-1.0);
    }

    if (model.y > win_rect.top()) || (model.y < win_rect.bottom()) {
        model.y_speed = model.y_speed * (-1.0);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);

    draw.ellipse()
        .x_y(model.x, model.y)
        .w_h(50.0, 50.0)
        .rgba(0.5, 0.5, 0.5, 1.0)
        .stroke(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
