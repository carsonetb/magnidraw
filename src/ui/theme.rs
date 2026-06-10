use std::{any::Any, collections::HashMap};

use crate::{
    Color,
    ui::{LabelParams, SeparatorParams, UIButtonParams},
};

/// A color theme to be used by UI nodes.
pub struct Theme {
    pub font: &'static str,
    pub clear_color: Color,
    pub buttons: UIButtonParams,
    pub separators: SeparatorParams,
    pub labels: LabelParams,
    /// Other attributes for custom UI elements.
    pub other: HashMap<String, Box<dyn Any>>,
}

impl Theme {
    pub fn nord() -> Self {
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
            other: HashMap::new(),
        }
    }

    pub fn catppuccin_latte() -> Self {
        let crust = Color::rgb255(220.0, 224.0, 232.0);
        let mantle = Color::rgb255(230.0, 233.0, 239.0);
        let base = Color::rgb255(239.0, 241.0, 245.0);
        let surface0 = Color::rgb255(204.0, 208.0, 218.0);
        let surface1 = Color::rgb255(188.0, 192.0, 204.0);
        let surface2 = Color::rgb255(172.0, 176.0, 190.0);
        let overlay0 = Color::rgb255(156.0, 160.0, 176.0);
        let overlay1 = Color::rgb255(140.0, 143.0, 161.0);
        let overlay2 = Color::rgb255(124.0, 127.0, 147.0);
        let subtext0 = Color::rgb255(108.0, 111.0, 133.0);
        let subtext1 = Color::rgb255(92.0, 95.0, 119.0);
        let text = Color::rgb255(76.0, 79.0, 105.0);

        Self {
            font: "Sans Serif",
            clear_color: base,
            buttons: UIButtonParams {
                text_color: text,
                color: surface0,
                hover_color: surface1,
                press_color: surface2,
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
                selection_color: Color::rgba(overlay2.r, overlay2.g, overlay2.b, 0.5),
            },
            other: HashMap::new(),
        }
    }
}
