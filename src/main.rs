#![windows_subsystem = "windows"]

mod asteroids;
mod nn;
mod player;
mod population;
mod skins;
mod world;

use nn::{ActivationFunc, NN};

use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets},
};
use population::{AutoSwitch, Population};
use tinyfiledialogs::{open_file_dialog, save_file_dialog};
use world::World;

fn window_conf() -> Conf {
    Conf {
        window_title: "Asteroids".to_string(),
        fullscreen: true,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as _);

    let pause = Texture2D::from_file_with_format(include_bytes!("../assets/pause.png"), None);
    let play = Texture2D::from_file_with_format(include_bytes!("../assets/play.png"), None);
    let fast = Texture2D::from_file_with_format(include_bytes!("../assets/fast.png"), None);
    let slow = Texture2D::from_file_with_format(include_bytes!("../assets/slow.png"), None);
    let restart = Texture2D::from_file_with_format(include_bytes!("../assets/restart.png"), None);

    next_frame().await;

    let SWIDTH: f32 = screen_width();
    let SHEIGHT: f32 = screen_height();
    let WIDTH: f32 = SWIDTH * (800. / 1400.);
    let HEIGHT: f32 = SHEIGHT * (780. / 800.);
    let th = (SHEIGHT - HEIGHT) * 0.5;

    let gamecam = Camera2D {
        zoom: vec2(2. / SWIDTH, -2. / SHEIGHT),
        offset: vec2((2. * th + WIDTH) / SWIDTH - 1., 0.),
        ..Default::default()
    };
    let netcam = Camera2D {
        zoom: vec2(2. / SWIDTH, -2. / SHEIGHT),
        offset: vec2((th + WIDTH) / SWIDTH, -((th + HEIGHT) * 0.5) / SHEIGHT),
        ..Default::default()
    };
    let statcam = Camera2D {
        zoom: vec2(2. / SWIDTH, -2. / SHEIGHT),
        offset: vec2((th + WIDTH) / SWIDTH, ((th + HEIGHT) * 0.5) / SHEIGHT),
        ..Default::default()
    };

    let mut speedup = 1;
    let mut paused = false;
    let mut bias = false;
    let mut human = false;
    let mut size: u32 = 100;
    let mut world: World = World::new(None, None, None, (WIDTH, HEIGHT));

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
        (WIDTH, HEIGHT),
    );

    let ui_thick = 34.;
    let nums = &[
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
    ];
    let skin = skins::get_ui_skin();
    let skin2 = skins::get_white_buttons_skin();
    let skin3 = skins::get_green_buttons_skin();

    root_ui().push_skin(&skin);
    loop {
        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(BLACK);
        set_camera(&gamecam);
        if !paused {
            for _ in 0..speedup {
                if !human {
                    pop.update((WIDTH, HEIGHT))
                } else if !world.over {
                    world.update((WIDTH, HEIGHT))
                };
            }
        }
        if human {
            world.draw(pop.debug);
            pop.draw_borders((WIDTH, HEIGHT, SWIDTH, SHEIGHT));
        } else {
            pop.draw((WIDTH, HEIGHT, SWIDTH, SHEIGHT));
        }
        draw_rectangle_lines(-WIDTH * 0.5, -HEIGHT * 0.5, WIDTH, HEIGHT, 2., WHITE);
        draw_rectangle_lines(
            WIDTH * 0.5 + th,
            -HEIGHT * 0.5,
            SWIDTH - WIDTH - 3. * th,
            ui_thick,
            2.,
            WHITE,
        );
        draw_rectangle_lines(
            WIDTH * 0.5 + th,
            -HEIGHT * 0.5 + (SHEIGHT - 3. * th) * 0.5 - ui_thick,
            SWIDTH - WIDTH - 3. * th,
            ui_thick,
            2.,
            WHITE,
        );

        set_camera(&netcam);
        pop.worlds[pop.track].player.draw_brain(
            SWIDTH - WIDTH - 3. * th,
            (SHEIGHT - 3. * th) * 0.5,
            bias,
        );
        set_camera(&statcam);
        let w = if human {
            &world
        } else {
            &pop.worlds[pop.track]
        };
        w.draw_stats(
            SWIDTH - WIDTH - 3. * th,
            (SHEIGHT - 7. * th) * 0.5 - 2. * ui_thick,
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

        let ui_width = SWIDTH - WIDTH - 3. * th + 1.;
        let ui_height = (SHEIGHT - 3. * th) * 0.5;
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
                        ui.label(vec2(ui_width - 371., 8.), &format!("{: >4}x", speedup));
                        ui.pop_skin();
                        // ui.same_line(242.);
                        ui.same_line(ui_width - 329.);
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
                                    (WIDTH, HEIGHT),
                                );
                                pop.worlds[0] = World::simulate(brain, (WIDTH, HEIGHT));
                            }
                        }
                        ui.same_line(0.);
                        if widgets::Button::new("Save Model").ui(ui) {
                            if let Some(path) = save_file_dialog("Save Model", "model.json") {
                                pop.worlds[pop.track].export_brain(&path);
                            }
                        }
                        ui.same_line(0.);
                        if widgets::Button::new(slow).ui(ui) || is_key_pressed(KeyCode::Z) {
                            speedup = std::cmp::max(speedup / 10, 1);
                        };
                        ui.same_line(0.);
                        if widgets::Button::new("1x").ui(ui) || is_key_pressed(KeyCode::X) {
                            speedup = 1;
                        };
                        ui.same_line(0.);
                        if widgets::Button::new(fast).ui(ui) || is_key_pressed(KeyCode::C) {
                            speedup = std::cmp::min(speedup * 10, 1000);
                        };
                        ui.same_line(0.);
                        if widgets::Button::new(if paused { play } else { pause }).ui(ui)
                            || is_key_pressed(KeyCode::P)
                        {
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
                        ui.label(Some(vec2(ui_width - 341., ui_thick * 0.5 - 7.)), "«Drag»");
                        ui.pop_skin();
                        ui.same_line(ui_width - 292.);
                        if widgets::Button::new(if pop.debug { "Debug:ON " } else { "Debug:OFF" })
                            .ui(ui)
                            || is_key_pressed(KeyCode::D)
                        {
                            pop.debug = !pop.debug;
                        };
                        ui.same_line(0.);
                        if widgets::Button::new(if bias { "Hide Bias" } else { "Show Bias" }).ui(ui)
                            || is_key_pressed(KeyCode::B)
                        {
                            bias = !bias;
                        };
                        ui.same_line(0.);
                        if widgets::Button::new(if !pop.focus { "Focus:OFF" } else { "Focus:ON " })
                            .ui(ui)
                            || is_key_pressed(KeyCode::F)
                        {
                            pop.focus = !pop.focus;
                        };
                        ui.same_line(0.);
                        if widgets::Button::new(restart).ui(ui) || is_key_pressed(KeyCode::R) {
                            if human {
                                world = World::new(None, None, None, (WIDTH, HEIGHT));
                            } else {
                                pop = Population::new(
                                    size as usize,
                                    auto_switch,
                                    hlayers.clone(),
                                    mut_rate,
                                    activs[activ],
                                    (WIDTH, HEIGHT),
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
                    } else if ui.button(None, "Best Alive") {
                        auto_switch = Some(AutoSwitch::BestAlive);
                        pop.auto_switch = auto_switch;
                    }
                    if auto_switch == Some(AutoSwitch::Best) {
                        ui.push_skin(&skin3);
                        ui.button(None, "Current #1");
                        ui.pop_skin();
                    } else if ui.button(None, "Current #1") {
                        auto_switch = Some(AutoSwitch::Best);
                        pop.auto_switch = auto_switch;
                    }
                    if auto_switch.is_none() {
                        ui.push_skin(&skin3);
                        ui.button(None, "Do Nothing");
                        ui.pop_skin();
                    } else if ui.button(None, "Do Nothing") {
                        auto_switch = None;
                        pop.auto_switch = auto_switch;
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
                            world = World::new(None, None, None, (WIDTH, HEIGHT));
                        } else {
                            pop = Population::new(
                                size as usize,
                                auto_switch,
                                hlayers.clone(),
                                mut_rate,
                                activs[activ],
                                (WIDTH, HEIGHT),
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
                            (WIDTH, HEIGHT),
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
