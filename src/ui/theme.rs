use crate::{
    Color,
    ui::{SeparatorParams, UIButtonParams},
};

#[derive(Clone, Copy)]
pub struct Theme {
    pub font: &'static str,
    pub clear_color: Color,
    pub buttons: UIButtonParams,
    pub separators: SeparatorParams,
}

impl Theme {
    pub fn nord() -> Self {
        let nord_0 = Color::rgb255(46.0, 52.0, 64.0);
        let nord_1 = Color::rgb255(59.0, 66.0, 82.0);
        let nord_2 = Color::rgb255(67.0, 76.0, 94.0);
        let nord_3 = Color::rgb255(76.0, 86.0, 106.0);

        let _nord_4 = Color::rgb255(216.0, 222.0, 233.0);
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
        }
    }
}
