use std::{any::Any, collections::HashMap};

use crate::{
    Color, Engine, EngineState, Scale,
    ui::{
        LabelParams, RadioParams, SeparatorParams, TextInputParams, ToggleParams, UIButtonParams,
    },
};

/// A color theme to be used by UI nodes.
pub struct Theme {
    pub font: &'static str,
    pub clear_color: Color,
    pub buttons: UIButtonParams,
    pub separators: SeparatorParams,
    pub labels: LabelParams,
    pub text_inputs: TextInputParams,
    pub toggles: ToggleParams,
    pub radios: RadioParams,
    /// Other attributes for custom UI elements.
    pub other: HashMap<String, Box<dyn Any>>,
}

impl Theme {
    pub fn nord(engine: &mut Engine, state: &mut EngineState) -> Self {
        let nord_0 = Color::rgb255(46.0, 52.0, 64.0);
        let nord_1 = Color::rgb255(59.0, 66.0, 82.0);
        let nord_2 = Color::rgb255(67.0, 76.0, 94.0);
        let nord_3 = Color::rgb255(76.0, 86.0, 106.0);

        let nord_4 = Color::rgb255(216.0, 222.0, 233.0);
        let _nord_5 = Color::rgb255(229.0, 233.0, 240.0);
        let nord_6 = Color::rgb255(236.0, 239.0, 244.0);

        Self {
            font: "Sans Serif",
            clear_color: nord_0,
            buttons: UIButtonParams {
                text_color: nord_6,
                color: nord_1,
                hover_color: nord_2,
                press_color: nord_1,
                padding: [10.0, 10.0, 10.0, 10.0],
                margin: [7.0, 7.0, 7.0, 7.0],
                radii: [10.0, 10.0, 10.0, 10.0],
                border_width: 0.0,
                border_color: Color::BLACK,
            },
            separators: SeparatorParams {
                width: 2.0,
                color: nord_3,
            },
            labels: LabelParams {
                color: nord_6,
                selection_color: Color::rgba(nord_4.r, nord_4.g, nord_4.b, 0.2),
            },
            text_inputs: TextInputParams {
                color: nord_1,
                text_color: nord_6,
                hint_color: nord_4,
                selection_color: Color::rgba(nord_4.r, nord_4.g, nord_4.b, 0.2),
                cursor_color: nord_6,
                cursor_width: 2.0,
                margin: [7.0, 7.0, 7.0, 7.0],
                padding: [5.0, 5.0, 5.0, 5.0],
                radii: [10.0, 10.0, 10.0, 10.0],
                border_width: 0.0,
                border_color: Color::BLACK,
            },
            toggles: ToggleParams {
                text_color: nord_6,
                toggle_on_color: nord_6,
                toggle_off_color: nord_6,
                toggle_on: engine.load_svg(
                    state,
                    include_bytes!("icons/toggle-right.svg"),
                    Scale::new(2.0, 2.0),
                ),
                toggle_off: engine.load_svg(
                    state,
                    include_bytes!("icons/toggle-left.svg"),
                    Scale::new(2.0, 2.0),
                ),
                margin: [7.0, 7.0, 7.0, 7.0],
                icon_text_padding: 7.0,
            },
            radios: RadioParams {
                text_color: nord_6,
                on_color: nord_6,
                off_color: nord_6,
                on_sprite: engine.load_svg(
                    state,
                    include_bytes!("icons/radio-button-fill.svg"),
                    Scale::new(2.0, 2.0),
                ),
                off_sprite: engine.load_svg(
                    state,
                    include_bytes!("icons/radio-button-light.svg"),
                    Scale::new(2.0, 2.0),
                ),
                margin: [7.0, 7.0, 7.0, 7.0],
                icon_text_padding: 7.0,
            },
            other: HashMap::new(),
        }
    }

    pub fn catppuccin_latte(engine: &mut Engine, state: &mut EngineState) -> Self {
        let crust = Color::rgb255(220.0, 224.0, 232.0);
        let _mantle = Color::rgb255(230.0, 233.0, 239.0);
        let base = Color::rgb255(239.0, 241.0, 245.0);
        let surface0 = Color::rgb255(204.0, 208.0, 218.0);
        let surface1 = Color::rgb255(188.0, 192.0, 204.0);
        let surface2 = Color::rgb255(172.0, 176.0, 190.0);
        let _overlay0 = Color::rgb255(156.0, 160.0, 176.0);
        let overlay1 = Color::rgb255(140.0, 143.0, 161.0);
        let overlay2 = Color::rgb255(124.0, 127.0, 147.0);
        let _subtext0 = Color::rgb255(108.0, 111.0, 133.0);
        let _subtext1 = Color::rgb255(92.0, 95.0, 119.0);
        let text = Color::rgb255(76.0, 79.0, 105.0);
        let green = Color::rgb255(64.0, 160.0, 43.0);
        let _red = Color::rgb255(210.0, 15.0, 57.0);
        let rosewater = Color::rgb255(220.0, 138.0, 120.0);

        engine.load_font(include_bytes!("fonts/opensans/OpenSans-Regular.ttf"));

        Self {
            font: "Open Sans",
            clear_color: base,
            buttons: UIButtonParams {
                text_color: text,
                color: surface0,
                hover_color: surface1,
                press_color: surface2,
                padding: [7.0, 7.0, 7.0, 7.0],
                margin: [7.0, 7.0, 7.0, 7.0],
                radii: [10.0, 10.0, 10.0, 10.0],
                border_width: 0.0,
                border_color: Color::BLACK,
            },
            separators: SeparatorParams {
                width: 2.0,
                color: crust,
            },
            labels: LabelParams {
                color: text,
                selection_color: Color::rgba(overlay2.r, overlay2.g, overlay2.b, 0.3),
            },
            text_inputs: TextInputParams {
                color: surface0,
                text_color: text,
                hint_color: overlay1,
                selection_color: Color::rgba(overlay2.r, overlay2.g, overlay2.b, 0.3),
                cursor_color: rosewater,
                cursor_width: 2.0,
                margin: [7.0, 7.0, 7.0, 7.0],
                padding: [10.0, 10.0, 5.0, 5.0],
                radii: [10.0, 10.0, 10.0, 10.0],
                border_width: 0.0,
                border_color: Color::BLACK,
            },
            toggles: ToggleParams {
                text_color: text,
                toggle_on_color: green,
                toggle_off_color: surface2,
                toggle_on: engine.load_svg(
                    state,
                    include_bytes!("icons/toggle-right.svg"),
                    Scale::new(1.3, 1.3),
                ),
                toggle_off: engine.load_svg(
                    state,
                    include_bytes!("icons/toggle-left.svg"),
                    Scale::new(1.3, 1.3),
                ),
                margin: [7.0, 7.0, -2.0, -2.0],
                icon_text_padding: 7.0,
            },
            radios: RadioParams {
                text_color: text,
                on_color: green,
                off_color: surface2,
                on_sprite: engine.load_svg(
                    state,
                    include_bytes!("icons/radio-button-fill.svg"),
                    Scale::new(1.3, 1.3),
                ),
                off_sprite: engine.load_svg(
                    state,
                    include_bytes!("icons/radio-button-light.svg"),
                    Scale::new(1.3, 1.3),
                ),
                margin: [7.0, 7.0, 7.0, 7.0],
                icon_text_padding: 7.0,
            },
            other: HashMap::new(),
        }
    }
}
