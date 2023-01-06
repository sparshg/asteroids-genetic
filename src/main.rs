mod asteroids;
mod nn;
mod player;
mod population;
mod world;

use macroquad::prelude::*;
use population::Population;

pub const WIDTH: f32 = 800.;
pub const HEIGHT: f32 = 780.;

fn window_conf() -> Conf {
    Conf {
        window_title: "Asteroids".to_string(),
        // fullscreen: true,
        window_width: 1400,
        window_height: 800,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as _);
    let th = (screen_height() - HEIGHT) * 0.5;

    let gamecam = Camera2D {
        zoom: vec2(2. / screen_width(), -2. / screen_height()),
        offset: vec2((2. * th + WIDTH) / screen_width() - 1., 0.),
        ..Default::default()
    };
    let maincam = Camera2D {
        zoom: vec2(2. / screen_width(), -2. / screen_height()),
        offset: vec2(
            (th + WIDTH) / screen_width(),
            -((th + HEIGHT) * 0.5) / screen_height(),
        ),
        ..Default::default()
    };
    // let mut cam = Camera2D::from_display_rect(Rect {
    //     x: 0.,
    //     y: 0.,
    //     w: 1600.,
    //     h: 1200.,
    // });
    // cam.offset = vec2(1., -1.);
    // // {
    //     zoom: vec2(2. / 800., -2. / 600.),
    //     // offset: vec2(-19. / 60., 0.),
    //     ..Default::default()
    // };
    let mut pop = Population::new(100);
    let mut speedup = false;
    loop {
        // set_camera(&cam);
        clear_background(BLACK);
        if is_key_pressed(KeyCode::S) {
            speedup = !speedup;
        }
        if speedup {
            for _ in 0..1000 {
                pop.update();
            }
        } else {
            pop.update();
            pop.draw();
        }
        draw_rectangle_lines(-WIDTH * 0.5, -HEIGHT * 0.5, WIDTH, HEIGHT, 2., WHITE);

        set_camera(&maincam);
        // draw_circle(0., 0., 20., RED);
        pop.worlds[0].see_brain().draw(
            screen_width() - WIDTH - 3. * th,
            (screen_height() - 3. * th) * 0.5,
        );
        set_camera(&gamecam);

        // set_camera(&maincam);
        // draw_texture_ex(
        //     target.texture,
        //     0.,
        //     0.,
        //     Color::new(1., 1., 1., 0.3),
        //     DrawTextureParams {
        //         flip_y: true,
        //         ..Default::default()
        //     },
        // );
        // set_camera(&cam);

        next_frame().await;
    }
}
