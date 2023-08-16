use notan::draw::*;
use notan::math::Vec2;
use notan::prelude::*;

const MOVE_SPEED: f32 = 100.0;

#[derive(AppState)]
struct State {
    font: Font,
    x: f32,
    y: f32,
    last_key: Option<KeyCode>,
    last_touch: Option<(f32, f32)>,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
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
        last_touch: None,
    }
}

fn update(app: &mut App, state: &mut State) {
    state.last_key = app.keyboard.last_key_released();

    let mut current_touch = None;
    app.touch.down.iter().for_each(|(&index, _)| {
        if current_touch.is_none() {
            if let Some((x, y)) = app.touch.position(index) {
                current_touch = Some((x, y));
            }
        }
    });

    if let (Some((x_old, y_old)), Some((x, y))) = (state.last_touch, current_touch) {
        state.x = (x - x_old) * MOVE_SPEED * app.timer.delta_f32();
        state.y = (y - y_old) * MOVE_SPEED * app.timer.delta_f32();
    }

    if app.keyboard.is_down(KeyCode::W) {
        state.y -= MOVE_SPEED * app.timer.delta_f32();
    }

    if app.keyboard.is_down(KeyCode::A) {
        state.x -= MOVE_SPEED * app.timer.delta_f32();
    }

    if app.keyboard.is_down(KeyCode::S) {
        state.y += MOVE_SPEED * app.timer.delta_f32();
    }

    if app.keyboard.is_down(KeyCode::D) {
        state.x += MOVE_SPEED * app.timer.delta_f32();
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    draw.circle(50.0)
        .position(state.x, state.y)
        .color(Color::RED);

    draw.text(&state.font, "Use WASD to move the circle")
        .position(10.0, 10.0)
        .size(20.0);

    if let Some(key) = &state.last_key {
        draw.text(&state.font, &format!("Last key: {key:?}"))
            .position(10.0, 560.0)
            .size(20.0);
    }

    gfx.render(&draw);
}
