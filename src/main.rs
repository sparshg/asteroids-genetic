mod asteroids;
mod nn;
mod player;
mod population;
mod world;

use nn::{ActivationFunc, NN};
use tinyfiledialogs::*;

use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets, Skin},
};
use population::{AutoSwitch, Population};
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
    let mut human = false;
    let mut size: u32 = 100;
    let mut world: World = World::new(None, None, None);

    let mut hlayers: Vec<usize> = vec![6, 6, 0];
    let mut prev_hlayers = hlayers.clone();

    let mut mut_rate = 0.05;
    let mut prev_mut_rate = 0.05;

    let mut activ: usize = 0;
    let mut prev_activ: usize = 0;
    let activs = [
        ActivationFunc::ReLU,
        ActivationFunc::Sigmoid,
        ActivationFunc::Tanh,
    ];
    let mut auto_switch = Some(AutoSwitch::BestAlive);

    let mut pop = Population::new(
        size as usize,
        auto_switch,
        hlayers.clone(),
        mut_rate,
        activs[activ],
    );

    let ui_thick = 34.;
    let nums = &[
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
    ];

    let skin = {
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
        let button_style = root_ui()
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
            .background_margin(RectOffset::new(1., 1., 1., 1.))
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
            // .text_color_hovered(LIGHTGRAY)
            // .text_color_clicked(WHITE)
            .build();
        let group_style = root_ui()
            .style_builder()
            .color(Color::new(1., 0., 0., 0.))
            .build();
        let editbox_style = root_ui()
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
            .background_margin(RectOffset::new(1., 1., 1., 1.))
            .text_color(WHITE)
            .build();
        let combobox_style = root_ui()
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
            .background_margin(RectOffset::new(1., 1., 1., 1.))
            .color_hovered(WHITE)
            .color_selected_hovered(WHITE)
            .color_inactive(WHITE)
            .color_clicked(WHITE)
            .color(WHITE)
            .build();

        Skin {
            window_style,
            button_style,
            label_style,
            group_style,
            editbox_style,
            combobox_style,
            margin: 0.,
            ..root_ui().default_skin()
        }
    };

    let mut skin2 = skin.clone();
    skin2.label_style = root_ui()
        .style_builder()
        .text_color(WHITE)
        .font_size(16)
        .text_color_hovered(LIGHTGRAY)
        .text_color_clicked(WHITE)
        .build();
    skin2.button_style = root_ui()
        .style_builder()
        .background(Image {
            width: 3,
            height: 3,
            bytes: vec![
                255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0,
                0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                255, 255,
            ],
        })
        .background_margin(RectOffset::new(1., 1., 1., 1.))
        .color_hovered(GREEN)
        .color_clicked(GREEN)
        .text_color(WHITE)
        .text_color_hovered(WHITE)
        .text_color_clicked(GREEN)
        .margin(RectOffset::new(4., 4., 2., 2.))
        .color_inactive(WHITE)
        .build();

    let mut skin3 = skin2.clone();
    skin3.button_style = root_ui()
        .style_builder()
        .background(Image {
            width: 3,
            height: 3,
            bytes: vec![
                0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 0, 0, 255, 0,
                255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255,
            ],
        })
        .background_margin(RectOffset::new(1., 1., 1., 1.))
        .color_hovered(GREEN)
        .color_clicked(GREEN)
        .text_color(GREEN)
        .text_color_hovered(GREEN)
        .text_color_clicked(GREEN)
        .margin(RectOffset::new(4., 4., 2., 2.))
        .color_inactive(GREEN)
        .build();

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
                if !human {
                    pop.update()
                } else if !world.over {
                    world.update()
                };
            }
        }
        if human {
            world.draw(pop.debug);
            pop.draw_borders();
        } else {
            pop.draw();
        }
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
        pop.worlds[pop.track].player.draw_brain(
            screen_width() - WIDTH - 3. * th,
            (screen_height() - 3. * th) * 0.5,
            bias,
        );
        set_camera(&statcam);
        let w = if human {
            &world
        } else {
            &pop.worlds[pop.track]
        };
        w.draw_stats(
            screen_width() - WIDTH - 3. * th,
            (screen_height() - 7. * th) * 0.5 - 2. * ui_thick,
            pop.worlds.iter().fold(1, |acc, w| {
                acc + if w.fitness > pop.worlds[pop.track].fitness {
                    1
                } else {
                    0
                }
            }),
        );
        if !pop.focus
            && is_mouse_button_pressed(MouseButton::Left)
            && mouse_position().0 < WIDTH + th
        {
            let (x, y) = mouse_position();
            pop.change_track(vec2(x - th - WIDTH * 0.5, y - th - HEIGHT * 0.5));
        }

        let ui_width = screen_width() - WIDTH - 3. * th + 1.;
        let ui_height = (screen_height() - 3. * th) * 0.5;
        root_ui().window(
            hash!(),
            vec2(WIDTH + 2. * th, th),
            vec2(ui_width, ui_height),
            |ui| {
                widgets::Group::new(hash!(), vec2(ui_width, ui_thick))
                    .position(vec2(0., 0.))
                    .ui(ui, |ui| {
                        ui.label(None, &format!("Generation: {}", pop.gen));
                        ui.push_skin(&skin2);
                        ui.label(vec2(200., 8.), &format!("{: >4}x", speedup));
                        ui.pop_skin();
                        ui.same_line(242.);
                        if widgets::Button::new("Load Model").ui(ui) {
                            if let Some(path) = open_file_dialog("Load Model", "model.json", None) {
                                let brain = NN::import(&path);
                                size = 1;
                                hlayers = brain
                                    .config
                                    .iter()
                                    .take(brain.config.len() - 1)
                                    .skip(1)
                                    .map(|x| x - 1)
                                    .collect::<Vec<_>>();
                                hlayers.resize(3, 0);
                                mut_rate = brain.mut_rate;
                                activ = activs.iter().position(|&x| x == brain.activ_func).unwrap();

                                prev_hlayers = hlayers.clone();
                                prev_mut_rate = mut_rate;
                                prev_activ = activ;

                                pop = Population::new(
                                    size as usize,
                                    auto_switch,
                                    hlayers.clone(),
                                    mut_rate,
                                    activs[activ],
                                );
                                pop.worlds[0] = World::simulate(brain);
                            }
                        }
                        ui.same_line(0.);
                        if widgets::Button::new("Save Model").ui(ui) {
                            if let Some(path) = save_file_dialog("Save Model", "model.json") {
                                pop.worlds[pop.track].export_brain(&path);
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
                widgets::Group::new(hash!(), vec2(ui_width, ui_thick))
                    .position(vec2(0., ui_height - ui_thick))
                    .ui(ui, |ui| {
                        ui.label(Some(vec2(0., 2.)), "Population:");
                        widgets::Group::new(hash!(), vec2(200., ui_thick))
                            .position(vec2(80., 0.))
                            .ui(ui, |ui| {
                                ui.drag(hash!(), "", Some((1, 300)), &mut size);
                            });
                        ui.push_skin(&skin2);
                        ui.label(Some(vec2(230., ui_thick * 0.5 - 7.)), "«Drag»");
                        ui.pop_skin();
                        ui.same_line(279.);
                        if widgets::Button::new(if pop.debug { "Debug:ON " } else { "Debug:OFF" })
                            .ui(ui)
                        {
                            pop.debug = !pop.debug;
                        };
                        ui.same_line(0.);
                        if widgets::Button::new(if bias { "Hide Bias" } else { "Show Bias" }).ui(ui)
                        {
                            bias = !bias;
                        };
                        ui.same_line(0.);
                        if widgets::Button::new(if !pop.focus { "Focus:OFF" } else { "Focus:ON " })
                            .ui(ui)
                        {
                            pop.focus = !pop.focus;
                        };
                        ui.same_line(0.);
                        if widgets::Button::new(restart).ui(ui) {
                            if human {
                                world = World::new(None, None, None);
                            } else {
                                pop = Population::new(
                                    size as usize,
                                    auto_switch,
                                    hlayers.clone(),
                                    mut_rate,
                                    activs[activ],
                                );
                            }
                        };
                    });
                ui.push_skin(&skin2);
                widgets::Group::new(
                    hash!(),
                    vec2(ui_width * 0.2, ui_height * 0.85 - 2. * th - 2. * ui_thick),
                )
                .position(vec2(ui_width * 0.82, ui_height * 0.15 + ui_thick + th))
                .ui(ui, |ui| {
                    ui.label(None, "Track Ship:");
                    ui.label(None, "(or click a");
                    ui.label(None, "ship in game)");

                    if ui.button(None, "Best Alive") {
                        pop.track_best(false);
                    }
                    if ui.button(None, "Current #1") {
                        pop.track_best(true);
                        auto_switch = Some(AutoSwitch::Best);
                        pop.auto_switch = auto_switch;
                    }
                    if ui.button(None, "LastGen #1") {
                        pop.track_prev_best();
                        auto_switch = None;
                        pop.auto_switch = auto_switch;
                    }
                    ui.label(None, " ");
                    ui.label(None, "Auto Switch");
                    ui.label(None, "When Dead to:");

                    if auto_switch == Some(AutoSwitch::BestAlive) {
                        ui.push_skin(&skin3);
                        ui.button(None, "Best Alive");
                        ui.pop_skin();
                    } else {
                        if ui.button(None, "Best Alive") {
                            auto_switch = Some(AutoSwitch::BestAlive);
                            pop.auto_switch = auto_switch;
                        }
                    }
                    if auto_switch == Some(AutoSwitch::Best) {
                        ui.push_skin(&skin3);
                        ui.button(None, "Current #1");
                        ui.pop_skin();
                    } else {
                        if ui.button(None, "Current #1") {
                            auto_switch = Some(AutoSwitch::Best);
                            pop.auto_switch = auto_switch;
                        }
                    }
                    if auto_switch.is_none() {
                        ui.push_skin(&skin3);
                        ui.button(None, "Do Nothing");
                        ui.pop_skin();
                    } else {
                        if ui.button(None, "Do Nothing") {
                            auto_switch = None;
                            pop.auto_switch = auto_switch;
                        }
                    }
                });
                widgets::Group::new(
                    hash!(),
                    vec2(ui_width * 0.2, ui_height * 0.85 - 2. * th - 2. * ui_thick),
                )
                .position(vec2(ui_width * 0.6 - th, ui_height * 0.15 + ui_thick + th))
                .ui(ui, |ui| {
                    ui.label(None, " ");
                    ui.push_skin(&skin);
                    if ui.button(
                        None,
                        if human {
                            "  Train  AI  "
                        } else {
                            "Play As Human"
                        },
                    ) {
                        human = !human;
                        if human {
                            world = World::new(None, None, None);
                        } else {
                            pop = Population::new(
                                size as usize,
                                auto_switch,
                                hlayers.clone(),
                                mut_rate,
                                activs[activ],
                            );
                        }
                    }
                    ui.pop_skin();
                    ui.label(None, "Mutation Rate");
                    ui.drag(hash!(), "«Drag»", Some((0., 1.)), &mut mut_rate);
                    if prev_mut_rate != mut_rate {
                        pop.change_mut(mut_rate);
                        prev_mut_rate = mut_rate;
                    }
                    ui.label(None, "Activation Func");
                    ui.combo_box(hash!(), "«Select»", &["ReLU", "Sigm", "Tanh"], &mut activ);
                    if prev_activ != activ {
                        pop.change_activ(activs[activ]);
                        prev_activ = activ;
                    }
                    ui.label(None, " ");
                    ui.label(None, "Hidden Layers");
                    ui.label(None, "Neurons Config");

                    ui.combo_box(hash!(), "Layer 1", nums, &mut hlayers[0]);
                    ui.combo_box(hash!(), "Layer 2", nums, &mut hlayers[1]);
                    ui.combo_box(hash!(), "Layer 3", nums, &mut hlayers[2]);
                    if prev_hlayers != hlayers {
                        pop = Population::new(
                            size as usize,
                            auto_switch,
                            hlayers.clone(),
                            mut_rate,
                            activs[activ],
                        );
                        prev_hlayers = hlayers.clone();
                    }
                });
                ui.pop_skin();
            },
        );
        next_frame().await;
    }
}
