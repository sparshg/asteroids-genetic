mod asteroids;
mod nn;
mod player;
mod population;
mod world;

use std::borrow::BorrowMut;

use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets, Skin},
};
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
    let pause = load_texture("pause.png").await.unwrap();
    let play = load_texture("play.png").await.unwrap();
    let fast = load_texture("fast.png").await.unwrap();
    let restart = load_texture("restart.png").await.unwrap();
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
    let mut paused = false;
    let mut checkbox = false;
    let mut combobox = 0;
    let mut text = String::new();
    let mut number = 0.0;

    let skin = {
        let boxed = root_ui()
            .style_builder()
            .background(Image {
                width: 3,
                height: 3,
                bytes: vec![
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255,
                ],
            })
            .background_margin(RectOffset::new(1., 1., 1., 1.));

        let window_style = root_ui()
            .style_builder()
            .background(Image {
                width: 1,
                height: 1,
                bytes: vec![0; 4],
            })
            .background_margin(RectOffset::new(0., 0., 0., 0.))
            .build();
        let button_style = boxed
            .color_hovered(RED)
            .color_clicked(BLUE)
            .text_color(WHITE)
            .text_color_hovered(WHITE)
            .text_color_clicked(WHITE)
            .margin(RectOffset::new(10., 10., 8., 8.))
            .build();
        let label_style = root_ui()
            .style_builder()
            .text_color(WHITE)
            .font_size(24)
            .margin(RectOffset::new(5., 5., 4., 4.))
            .build();

        Skin {
            window_style,
            button_style,
            label_style,
            margin: 0.,
            ..root_ui().default_skin()
        }
    };

    root_ui().push_skin(&skin);
    loop {
        clear_background(BLACK);
        if is_key_pressed(KeyCode::S) {
            speedup = !speedup;
        }
        if speedup {
            if !paused {
                for _ in 0..1000 {
                    pop.update();
                }
            }
        } else {
            if !paused {
                pop.update();
            }
            pop.draw();
        }
        draw_rectangle_lines(-WIDTH * 0.5, -HEIGHT * 0.5, WIDTH, HEIGHT, 2., WHITE);
        draw_rectangle_lines(
            WIDTH * 0.5 + th,
            -HEIGHT * 0.5,
            screen_width() - WIDTH - 3. * th,
            34.,
            2.,
            WHITE,
        );
        draw_rectangle_lines(
            WIDTH * 0.5 + th,
            -HEIGHT * 0.5 + (screen_height() - 3. * th) * 0.5 - 34.,
            screen_width() - WIDTH - 3. * th,
            34.,
            2.,
            WHITE,
        );

        set_camera(&maincam);
        // draw_circle(0., 0., 20., RED);
        pop.worlds[0].player.draw_brain(
            screen_width() - WIDTH - 3. * th,
            (screen_height() - 3. * th) * 0.5,
        );

        set_camera(&gamecam);
        root_ui().window(
            hash!(),
            vec2(WIDTH + 2. * th, th),
            vec2(screen_width() - WIDTH - 3. * th + 1., 34.),
            |ui| {
                ui.label(None, &format!("Generation: {}", pop.gen));
                ui.same_line(278.);
                widgets::Button::new("Load Model").ui(ui);
                ui.same_line(0.);
                widgets::Button::new("Save Model").ui(ui);
                ui.same_line(0.);
                if widgets::Button::new(fast).ui(ui) {
                    speedup = !speedup;
                };
                ui.same_line(0.);
                if widgets::Button::new(restart).ui(ui) {
                    pop = Population::new(100);
                };
                ui.same_line(0.);
                if widgets::Button::new(if paused { play } else { pause }).ui(ui) {
                    paused = !paused;
                };
            },
        );
        root_ui().window(
            hash!(),
            vec2(WIDTH + 2. * th, (screen_height() - th) * 0.5 - 34.),
            vec2(screen_width() - WIDTH - 3. * th + 1., 34.),
            |ui| {
                ui.label(None, &format!("Generation: {}", pop.gen));
                ui.same_line(278.);
                widgets::Button::new("Load Model").ui(ui);
                ui.same_line(0.);
                widgets::Button::new("Save Model").ui(ui);
                ui.same_line(0.);
                if widgets::Button::new(fast).ui(ui) {
                    speedup = !speedup;
                };
                ui.same_line(0.);
                if widgets::Button::new(restart).ui(ui) {
                    pop = Population::new(100);
                };
                ui.same_line(0.);
                if widgets::Button::new(if paused { play } else { pause }).ui(ui) {
                    paused = !paused;
                };
            },
        );

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
