use macroquad::{prelude::*, ui::*};

pub fn get_ui_skin() -> Skin {
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
                255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0,
                0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                255, 255,
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
                255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0,
                0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                255, 255,
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
                255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0,
                0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                255, 255,
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
}

pub fn get_white_buttons_skin() -> Skin {
    let mut skin2 = get_ui_skin();
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
    skin2
}

pub fn get_green_buttons_skin() -> Skin {
    let mut skin3 = get_white_buttons_skin();
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
    skin3
}
