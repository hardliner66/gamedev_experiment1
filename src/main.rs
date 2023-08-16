use notan::draw::*;
use notan::egui::{self, *};
use notan::math::Mat3;
use notan::prelude::*;

const MOVE_SPEED: f32 = 100.0;

#[derive(AppState)]
struct State {
    font: Font,
    x: f32,
    y: f32,
    last_key: Option<KeyCode>,
}

fn setup(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("../assets/Ubuntu-B.ttf"))
        .unwrap();

    State {
        font,
        x: 400.0,
        y: 300.0,
        last_key: None,
    }
}

fn sign((p1_x, p1_y): (f32, f32), (p2_x, p2_y): (f32, f32), (p3_x, p3_y): (f32, f32)) -> f32 {
    return (p1_x - p3_x) * (p2_y - p3_y) - (p2_x - p3_x) * (p1_y - p3_y);
}

fn point_in_triangle(pt: (f32, f32), v1: (f32, f32), v2: (f32, f32), v3: (f32, f32)) -> bool {
    let d1 = sign(pt, v1, v2);
    let d2 = sign(pt, v2, v3);
    let d3 = sign(pt, v3, v1);

    let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
    let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);

    return !(has_neg && has_pos);
}

fn move_up(app: &mut App, state: &mut State) {
    state.y -= MOVE_SPEED * app.timer.delta_f32();
}

fn move_left(app: &mut App, state: &mut State) {
    state.x -= MOVE_SPEED * app.timer.delta_f32();
}

fn move_down(app: &mut App, state: &mut State) {
    state.y += MOVE_SPEED * app.timer.delta_f32();
}

fn move_right(app: &mut App, state: &mut State) {
    state.x += MOVE_SPEED * app.timer.delta_f32();
}

fn update(app: &mut App, state: &mut State) {
    state.last_key = app.keyboard.last_key_released();

    let mut up_pressed = false;
    let mut down_pressed = false;
    let mut left_pressed = false;
    let mut right_pressed = false;
    app.touch.down.iter().for_each(|(&index, _)| {
        if let Some((x, y)) = app.touch.position(index) {
            if point_in_triangle((x, y), LEFT_P1, LEFT_P2, LEFT_P3) {
                left_pressed = true;
            }
            if point_in_triangle((x, y), RIGHT_P1, RIGHT_P2, RIGHT_P3) {
                right_pressed = true;
            }
            if point_in_triangle((x, y), DOWN_P1, DOWN_P2, DOWN_P3) {
                down_pressed = true;
            }
            if point_in_triangle((x, y), UP_P1, UP_P2, UP_P3) {
                up_pressed = true;
            }
        }
    });

    if app.mouse.left_is_down() {
        let (x, y) = app.mouse.position();
        if point_in_triangle((x, y), LEFT_P1, LEFT_P2, LEFT_P3) {
            left_pressed = true;
        }
        if point_in_triangle((x, y), RIGHT_P1, RIGHT_P2, RIGHT_P3) {
            right_pressed = true;
        }
        if point_in_triangle((x, y), DOWN_P1, DOWN_P2, DOWN_P3) {
            down_pressed = true;
        }
        if point_in_triangle((x, y), UP_P1, UP_P2, UP_P3) {
            up_pressed = true;
        }
    }

    if up_pressed {
        move_up(app, state);
    }
    if down_pressed {
        move_down(app, state);
    }
    if right_pressed {
        move_right(app, state);
    }
    if left_pressed {
        move_left(app, state);
    }

    if app.keyboard.is_down(KeyCode::W) {
        move_up(app, state);
    }

    if app.keyboard.is_down(KeyCode::A) {
        move_left(app, state);
    }

    if app.keyboard.is_down(KeyCode::S) {
        move_down(app, state);
    }

    if app.keyboard.is_down(KeyCode::D) {
        move_right(app, state);
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    let win = WindowConfig::new()
        .size(500, 500)
        .vsync(true)
        .high_dpi(true);

    notan::init_with(setup)
        .add_config(win)
        .add_config(EguiConfig)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

macro_rules! add_points {
    (($x1: expr, $y1: expr), $o: expr) => {
        ($x1 + $o.0, $y1 + $o.1)
    };
}

const LEFT_OFFSET: (f32, f32) = (395.0, 425.0);
const LEFT_P1: (f32, f32) = add_points!((0.0, 25.0), LEFT_OFFSET);
const LEFT_P2: (f32, f32) = add_points!((0.0, -25.0), LEFT_OFFSET);
const LEFT_P3: (f32, f32) = add_points!((-30.0, 0.0), LEFT_OFFSET);

const RIGHT_OFFSET: (f32, f32) = (455.0, 425.0);
const RIGHT_P1: (f32, f32) = add_points!((0.0, 25.0), RIGHT_OFFSET);
const RIGHT_P2: (f32, f32) = add_points!((0.0, -25.0), RIGHT_OFFSET);
const RIGHT_P3: (f32, f32) = add_points!((30.0, 0.0), RIGHT_OFFSET);

const UP_OFFSET: (f32, f32) = (425.0, 395.0);
const UP_P1: (f32, f32) = add_points!((25.0, 0.0), UP_OFFSET);
const UP_P2: (f32, f32) = add_points!((-25.0, 0.0), UP_OFFSET);
const UP_P3: (f32, f32) = add_points!((0.0, -30.0), UP_OFFSET);

const DOWN_OFFSET: (f32, f32) = (425.0, 455.0);
const DOWN_P1: (f32, f32) = add_points!((25.0, 0.0), DOWN_OFFSET);
const DOWN_P2: (f32, f32) = add_points!((-25.0, 0.0), DOWN_OFFSET);
const DOWN_P3: (f32, f32) = add_points!((0.0, 30.0), DOWN_OFFSET);

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    draw.circle(50.0)
        .position(state.x, state.y)
        .color(Color::RED);

    draw.triangle(LEFT_P1, LEFT_P2, LEFT_P3);
    draw.triangle(RIGHT_P1, RIGHT_P2, RIGHT_P3);
    draw.triangle(UP_P1, UP_P2, UP_P3);
    draw.triangle(DOWN_P1, DOWN_P2, DOWN_P3);

    draw.text(&state.font, "Use WASD to move the circle")
        .position(10.0, 10.0)
        .size(20.0);

    if let Some(key) = &state.last_key {
        draw.text(&state.font, &format!("Last key: {key:?}"))
            .position(10.0, 460.0)
            .size(20.0);
    }

    gfx.render(&draw);
}
