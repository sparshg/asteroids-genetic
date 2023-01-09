mod asteroids;
mod nn;
mod player;
mod population;
mod world;

use nn::NN;
use tinyfiledialogs::*;

use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets, Skin},
};
use population::Population;
use world::World;

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
    let pause = load_texture("assets/pause.png").await.unwrap();
    let play = load_texture("assets/play.png").await.unwrap();
    let fast = load_texture("assets/fast.png").await.unwrap();
    let slow = load_texture("assets/slow.png").await.unwrap();
    let restart = load_texture("assets/restart.png").await.unwrap();
    let th = (screen_height() - HEIGHT) * 0.5;

    let gamecam = Camera2D {
        zoom: vec2(2. / screen_width(), -2. / screen_height()),
        offset: vec2((2. * th + WIDTH) / screen_width() - 1., 0.),
        ..Default::default()
    };
    let netcam = Camera2D {
        zoom: vec2(2. / screen_width(), -2. / screen_height()),
        offset: vec2(
            (th + WIDTH) / screen_width(),
            -((th + HEIGHT) * 0.5) / screen_height(),
        ),
        ..Default::default()
    };
    let statcam = Camera2D {
        zoom: vec2(2. / screen_width(), -2. / screen_height()),
        offset: vec2(
            (th + WIDTH) / screen_width(),
            ((th + HEIGHT) * 0.5) / screen_height(),
        ),
        ..Default::default()
    };

    let mut speedup = 1;
    let mut paused = false;
    let mut bias = false;
    let mut size = 100;
    let mut pop = Population::new(size as usize);

    let ui_thick = 34.;

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
            .color_inactive(WHITE)
            .build();
        let button_style = boxed
            .color_hovered(RED)
            .color_clicked(BLUE)
            .text_color(WHITE)
            .text_color_hovered(WHITE)
            .text_color_clicked(WHITE)
            .margin(RectOffset::new(10., 10., 8., 8.))
            .color_inactive(WHITE)
            .build();
        let label_style = root_ui()
            .style_builder()
            .text_color(WHITE)
            .font_size(24)
            .margin(RectOffset::new(5., 5., 4., 4.))
            .color_inactive(WHITE)
            .build();
        let group_style = root_ui()
            .style_builder()
            .color(Color::new(0., 0., 0., 0.))
            .build();

        Skin {
            window_style,
            button_style,
            label_style,
            group_style,

            margin: 0.,
            ..root_ui().default_skin()
        }
    };

    root_ui().push_skin(&skin);
    loop {
        clear_background(BLACK);
        set_camera(&gamecam);
        if is_key_pressed(KeyCode::S) {
            speedup = (speedup * 10) % 9999;
        }
        if is_key_pressed(KeyCode::P) {
            paused = !paused;
        }
        if !paused {
            for _ in 0..speedup {
                pop.update();
            }
        }
        pop.draw();
        draw_rectangle_lines(-WIDTH * 0.5, -HEIGHT * 0.5, WIDTH, HEIGHT, 2., WHITE);
        draw_rectangle_lines(
            WIDTH * 0.5 + th,
            -HEIGHT * 0.5,
            screen_width() - WIDTH - 3. * th,
            ui_thick,
            2.,
            WHITE,
        );
        draw_rectangle_lines(
            WIDTH * 0.5 + th,
            -HEIGHT * 0.5 + (screen_height() - 3. * th) * 0.5 - ui_thick,
            screen_width() - WIDTH - 3. * th,
            ui_thick,
            2.,
            WHITE,
        );

        set_camera(&netcam);
        pop.worlds[0].player.draw_brain(
            screen_width() - WIDTH - 3. * th,
            (screen_height() - 3. * th) * 0.5,
            bias,
        );
        set_camera(&statcam);
        pop.worlds[0].draw_stats(
            screen_width() - WIDTH - 3. * th,
            (screen_height() - 7. * th) * 0.5 - 2. * ui_thick,
        );
        if is_mouse_button_pressed(MouseButton::Left) && mouse_position().0 < WIDTH + th {
            let (x, y) = mouse_position();
            for i in 0..pop.worlds.len() {
                if (pop.worlds[i].player.pos - vec2(x - th - WIDTH * 0.5, y - th - HEIGHT * 0.5))
                    .length_squared()
                    < 256.
                {
                    pop.worlds.swap(0, i);
                    pop.worlds[0].track(true);
                    pop.worlds[i].track(false);
                    break;
                }
            }
        }

        let ui_width = screen_width() - WIDTH - 3. * th + 1.;
        let ui_height = (screen_height() - 3. * th) * 0.5;
        root_ui().window(
            hash!(),
            vec2(WIDTH + 2. * th, th),
            vec2(ui_width, ui_height),
            |ui| {
                widgets::Group::new(hash!(), Vec2::new(ui_width, ui_thick))
                    .position(Vec2::new(0., 0.))
                    .ui(ui, |ui| {
                        ui.label(None, &format!("Generation: {}", pop.gen));
                        ui.same_line(242.);
                        if widgets::Button::new("Load Model").ui(ui) {
                            if let Some(path) = open_file_dialog("Load Model", "model.json", None) {
                                let brain = NN::import(&path);
                                size = 1;
                                pop = Population::new(1);
                                pop.worlds[0] = World::simulate(Some(brain));
                            }
                        }
                        ui.same_line(0.);
                        if widgets::Button::new("Save Model").ui(ui) {
                            if let Some(path) = save_file_dialog("Save Model", "model.json") {
                                pop.worlds[0].export_brain(&path);
                            }
                        }
                        ui.same_line(0.);
                        if widgets::Button::new(slow).ui(ui) {
                            speedup = std::cmp::max(speedup / 10, 1);
                        };
                        ui.same_line(0.);
                        if widgets::Button::new("1x").ui(ui) {
                            speedup = 1;
                        };
                        ui.same_line(0.);
                        if widgets::Button::new(fast).ui(ui) {
                            speedup = std::cmp::min(speedup * 10, 1000);
                        };
                        ui.same_line(0.);
                        if widgets::Button::new(if paused { play } else { pause }).ui(ui) {
                            paused = !paused;
                        };
                    });
                widgets::Group::new(hash!(), Vec2::new(ui_width, ui_thick))
                    .position(Vec2::new(0., ui_height - ui_thick))
                    .ui(ui, |ui| {
                        ui.label(Some(vec2(0., 2.)), "«Population»");
                        widgets::Group::new(hash!(), Vec2::new(100., ui_thick))
                            .position(Vec2::new(140., 0.))
                            .ui(ui, |ui| {
                                ui.drag(hash!(), "", Some((1, 300)), &mut size);
                            });
                        ui.same_line(307.);
                        if widgets::Button::new("Debug").ui(ui) {
                            pop.debug = !pop.debug;
                        };
                        ui.same_line(0.);
                        if widgets::Button::new(if bias { "Hide Bias" } else { "Show Bias" }).ui(ui)
                        {
                            bias = !bias;
                        };
                        ui.same_line(0.);
                        if widgets::Button::new(if !pop.focus { "Show Best" } else { "Show All " })
                            .ui(ui)
                        {
                            pop.focus = !pop.focus;
                        };
                        ui.same_line(0.);
                        if widgets::Button::new(restart).ui(ui) {
                            pop = Population::new(size as usize);
                        };
                    });
            },
        );
        next_frame().await;
    }
}
