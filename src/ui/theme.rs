use crate::{Color, ui::UIButtonParams};

#[derive(Clone, Copy)]
pub struct Theme {
    pub font: &'static str,
    pub clear_color: Color,
    pub buttons: UIButtonParams,
}

impl Theme {
    const NORD_0: Color = Color::rgb255(46.0, 52.0, 64.0);
    const NORD_1: Color = Color::rgb255(59.0, 66.0, 82.0);
    const NORD_2: Color = Color::rgb255(67.0, 76.0, 94.0);
    const NORD_3: Color = Color::rgb255(76.0, 86.0, 106.0);

    const NORD_4: Color = Color::rgb255(216.0, 222.0, 233.0);
    const NORD_5: Color = Color::rgb255(229.0, 233.0, 240.0);
    const NORD_6: Color = Color::rgb255(236.0, 239.0, 244.0);

    pub fn nord() -> Self {
        Self {
            font: "Sans Serif",
            clear_color: Self::NORD_0,
            buttons: UIButtonParams {
                text_color: Self::NORD_6,
                color: Self::NORD_1,
                hover_color: Self::NORD_2,
                press_color: Self::NORD_1,
                radii: [10.0, 10.0, 10.0, 10.0],
                border_width: 0.0,
                border_color: Color::BLACK,
            },
        }
    }
}
