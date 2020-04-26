use nannou::prelude::*;
use rand::Rng;

static WINDOW_WIDTH: u32 = 1280;
static WINDOW_HEIGHT: u32 = 720;
static LINE_LENGTH: f32 = 200.0;
static POINT_COUNT: u32 = 100;

lazy_static::lazy_static! {
static ref BACKGROUND: LinSrgba = Srgba::new(0.0, 0.05, 0.1, 1.0).into_linear();
static ref LINE: LinSrgba = Srgba::new(0.0, 0.4, 0.6, 1.0).into_linear();
}

fn main() {
    std::env::set_var("WINIT_X11_SCALE_FACTOR", "1");
    nannou::app(init).update(update).simple_window(render).run();
}

struct Model {
    pub points: Vec<Point>,
}

#[derive(Default, Debug, PartialEq, Copy, Clone)]
struct Point {
    pub loc: Point2,
    pub vel: Vector2,
}

fn init(app: &App) -> Model {
    app.main_window().set_fullscreen(true);
    let mut points = vec![];
    for _i in 0..POINT_COUNT {
        let mut rng = rand::thread_rng();
        points.push(Point {
            loc: Point2::new(
                rng.gen_range(-((WINDOW_WIDTH / 2) as f32) - LINE_LENGTH, (WINDOW_WIDTH / 2) as f32 + LINE_LENGTH),
                rng.gen_range(-((WINDOW_HEIGHT / 2) as f32) - LINE_LENGTH, (WINDOW_HEIGHT / 2) as f32 + LINE_LENGTH),
            ),
            vel: Vector2::from_angle(rng.gen_range(0f32, PI)) * rng.gen_range(0.5f32, 2.0),
        });
    }
    Model { points }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let window_width = app.window_rect().w();
    let window_height = app.window_rect().h();

    for point in model.points.iter_mut() {
        point.loc += point.vel;

        let x_bound = window_width / 2.0 + LINE_LENGTH;
        let y_bound = window_height / 2.0 + LINE_LENGTH;

        if point.loc.x < -x_bound {
            point.loc.x += window_width + LINE_LENGTH * 2.0;
        } else if point.loc.x > x_bound {
            point.loc.x -= window_width + LINE_LENGTH * 2.0;
        }

        if point.loc.y < -y_bound {
            point.loc.y += window_height + LINE_LENGTH * 2.0;
        } else if point.loc.y > y_bound {
            point.loc.y -= window_height + LINE_LENGTH * 2.0;
        }
    }
}

fn render(app: &App, model: &Model, frame: Frame) {
    frame.clear(BACKGROUND.clone());

    let draw = app.draw();
    let mut gathered: Vec<Point> = vec![];

    for point in model.points.iter() {
        for other in gathered.iter() {
            let distance = point.loc.distance(other.loc);
            if distance < LINE_LENGTH {
                let weight = 1.0 - distance / LINE_LENGTH;
                let mut color = LINE.clone();
                color.alpha = weight;
                draw.line()
                    .color(color)
                    .start(point.loc)
                    .end(other.loc)
                    .finish();
            }
        }
        gathered.push(*point);
    }

    draw.to_frame(app, &frame).unwrap();
}
